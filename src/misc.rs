use afire::{Content, Response};
use serde_json::json;

pub fn json_err(err: &str) -> Response {
    Response::new()
        .status(400)
        .text(json!({ "error": err }))
        .content(Content::JSON)
}
