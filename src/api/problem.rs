use afire::{Method, Response, Server};
use serde_json::json;

use crate::{problem::Languge, App};

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

        let pub_cases = problem
            .cases
            .iter()
            .take(problem.tags.show_cases.unwrap_or(4))
            .collect::<Vec<_>>();

        Response::new().text(json!({
            "name": problem.name,
            "text": problem.document,
            "hint": problem.hint,
            "baseCode": problem.base_code,
            "cases": pub_cases,
            "moreCases": problem.cases.len() > pub_cases.len(),
            "lang": problem.tags.lang.unwrap_or(Languge::Java),
            "section": problem.tags.section
        }))
    });
}
