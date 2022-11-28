use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

use afire::{Content, Method, Response, Server};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::problem::{Language, Problem};
use crate::r#const::LANGS;
use crate::{langs, App};

#[derive(Deserialize)]
struct RouteData {
    problem: String,
    code: String,
}

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/run", |app, req| {
        let data = serde_json::from_str::<RouteData>(&String::from_utf8_lossy(&req.body)).unwrap();
        let problem = match app.problems.get(&data.problem) {
            Some(i) => i,
            None => {
                return Response::new()
                    .status(404)
                    .text(json!({"error": "Problem not found"}))
            }
        };

        // Get languge
        let language = LANGS
            .get(&problem.tags.lang.unwrap_or(Language::Java).runner())
            .unwrap();
        let info = &langs::LANGS[language.info_index];

        // Write code to disk
        let mut code_file = tempfile::NamedTempFile::new_in(&app.config.tmp_folder).unwrap();
        code_file
            .write_all(info.run_file(&data.code).as_bytes())
            .unwrap();

        // Data
        // [6-digit code];[func name];i,i,i>o;i,i,i>o
        let shared_token = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(6)
            .map(|x| x as char)
            .collect::<String>();
        let data = format!(
            "{};{};{}",
            shared_token,
            problem.func_name,
            problem.stringify()
        );

        // Build and run in a docker container
        let time = Instant::now();
        let run = Command::new(&app.config.docker_command)
            .args([
                "run",
                "--rm",
                "--cap-drop=ALL",
                "--security-opt=no-new-privileges",
                "--net",
                "none",
                "--memory",
                "128m",
                "--memory-swap",
                "256m",
                "--pids-limit",
                "512",
                "-v",
                &format!(
                    "{}:/runner/{}",
                    code_file.path().to_string_lossy(),
                    language.source_path
                ),
                "-e",
                &format!("TIMEOUT={}", &app.config.docker_timeout),
                "-e",
                &format!("DATA={}", urlencoding::encode(&data)),
                &language.image_name,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();
        let time = time.elapsed().as_millis() as u64;

        // Get outputs
        let out = String::from_utf8_lossy(&run.stdout);
        let err = String::from_utf8_lossy(&run.stderr);

        // Parse output
        let (output, clean_out) = parse_output(problem, &shared_token, &err);
        dbg!(&output, &clean_out);

        // TODO: maybe only send results of the shown test cases
        // also denote which test cases have passed
        Response::new()
            .text(json!({
                "stdout": !out.is_empty(),
                "stderr": clean_out,
                "time": time,
                "result": output,
            }))
            .content(Content::JSON)
    });
}

/// Vector of length problems with the value being the value
/// being the user's program output.
type RunCases = Vec<String>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "data")]
enum RunOutput {
    /// Program ran successfully and tests passed
    Success(RunCases),
    /// Program ran successfully but some tests failed    
    Fail(RunCases),
    /// Program failed to run
    Error(ErrorKind),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ErrorKind {
    /// Function not found
    FunctionDefNotFound,
    /// Invalid function signature
    InvalidFuncSig,
}

/// -> (RunOutput, [clean stderr])
fn parse_output(problem: &Problem, token: &str, std_err: &str) -> (RunOutput, String) {
    let mut cleaned = Vec::new();
    let mut force_error = None;
    let mut run_cases = Vec::with_capacity(problem.cases.len());
    let prefix = format!("{};", token);

    for i in std_err.lines() {
        if let Some(j) = i.strip_prefix(&prefix) {
            if force_error.is_some() {
                continue;
            }

            let parts = j.split_once(';').unwrap();
            match parts.0 {
                "ERROR" => match parts.1 {
                    "FUNC_DEF_NOT_FOUND" => {
                        force_error = Some(RunOutput::Error(ErrorKind::FunctionDefNotFound))
                    }
                    "INVALID_FUNC_SIG" => {
                        force_error = Some(RunOutput::Error(ErrorKind::InvalidFuncSig))
                    }
                    _ => panic!("Unknown error type: {}", parts.1),
                },
                "RESULT" => {
                    let parts = parts.1.split(';').collect::<Vec<_>>();
                    debug_assert!(parts.len() == problem.cases.len() * 2);

                    for i in 0..problem.cases.len() {
                        let result = parts[i + problem.cases.len()];
                        let status = match parts[i] {
                            "P" => true,
                            "F" => false,
                            _ => panic!("Invalid result state: {}", parts[i]),
                        };

                        run_cases.push((result.to_string(), status));
                    }
                }
                _ => panic!("Unknown message type: {}", parts.0),
            }
            continue;
        }

        cleaned.push(i);
    }

    if let Some(i) = force_error {
        return (i, cleaned.join("\n"));
    }

    let all_success = run_cases.iter().all(|(_, i)| *i);
    let output = run_cases.into_iter().map(|(i, _)| i).collect();

    (
        match all_success {
            true => RunOutput::Success(output),
            false => RunOutput::Fail(output),
        },
        cleaned.join("\n"),
    )
}
