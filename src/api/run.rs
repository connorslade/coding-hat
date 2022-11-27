use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

use afire::{Content, Method, Response, Server};
use rand::Rng;
use serde::Deserialize;
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
        let output = parse_output(&problem, &shared_token, &err);

        Response::new()
            .text(json!({
                "stdout": out,
                "stderr": err,
                "time": time
            }))
            .content(Content::JSON)
    });
}

/// Vector of length problems with the value being the value
/// being the user's program output.
type RunCases = Vec<String>;

enum RunOutput {
    /// Program ran successfully and tests passed
    Success(RunCases),

    /// Program ran successfully but some tests failed    
    Fail(RunCases),

    /// Program failed to run
    Error(ErrorKind),
}

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
    let mut run_cases = vec![(false, String::new()); problem.cases.len()];
    let prefix = format!("{};", token);

    for i in std_err.lines() {
        if let Some(j) = i.strip_prefix(&prefix) {
            let mut parts = j.splitn(2, ';');
            let msg_type = parts.next().unwrap();
            let data = parts.next().unwrap();

            match msg_type {
                "ERROR" if force_error.is_none() => match data {
                    "FUNC_DEF_NOT_FOUND" => {
                        force_error = Some(RunOutput::Error(ErrorKind::FunctionDefNotFound))
                    }
                    "INVALID_FUNC_SIG" => {
                        force_error = Some(RunOutput::Error(ErrorKind::InvalidFuncSig))
                    }
                    _ => panic!("Unknown error type: {}", data),
                },
                "RESULT" => {
                    let mut parts = data.split(';');
                    for i in 0..problem.cases.len() {
                        let part = parts.next().unwrap();
                        match part {
                            "P" => run_cases[i].0 = true,
                            "F" => run_cases[i].0 = false,
                            _ => panic!("Invalid result state: {}", part),
                        }
                    }

                    for i in 0..problem.cases.len() {
                        run_cases[i].1 = parts.next().unwrap().to_string();
                    }
                }
                _ => panic!("Unknown message type: {}", msg_type),
            }

            continue;
        }

        cleaned.push(i);
    }

    todo!()
}
