use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

use afire::{Content, Method, Response, Server};
use rand::Rng;
use serde::Deserialize;
use serde_json::json;

use crate::problem::Language;
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
        println!("{}", data);

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

        // Send response
        let out = String::from_utf8_lossy(&run.stdout);
        let err = String::from_utf8_lossy(&run.stderr);
        println!("\n{err}");

        Response::new()
            .text(json!({
                "stdout": out,
                "stderr": err,
                "time": time
            }))
            .content(Content::JSON)
    });
}
