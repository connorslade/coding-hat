use afire::{Method, Response, Server};
use serde_json::json;

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/api/problem/{id}", |app, req| {
        let id = req.path_param("id").unwrap();
        let problem = match app.problems.get(&id) {
            Some(i) => i,
            None => {
                return Response::new()
                    .status(404)
                    .text(json!({"error": "Problem not found"}))
            }
        };

        let pub_cases = problem.public_cases();
        Response::new().text(json!({
            "name": problem.name,
            "text": problem.document,
            "hint": problem.hint,
            "cases": pub_cases,
            "moreCases": problem.cases.len() > pub_cases.len(),
            "lang": problem.lang()
        }))
    });
}
