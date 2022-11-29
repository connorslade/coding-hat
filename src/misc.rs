use afire::{Content, Cookie, Request, Response};
use serde_json::json;

pub fn json_err(err: &str) -> Response {
    Response::new()
        .status(400)
        .text(json!({ "error": err }))
        .content(Content::JSON)
}

pub fn get_cookie(req: &Request, name: &str) -> Option<Cookie> {
    req.cookies
        .iter()
        .find(|x| x.name == name)
        .map(|x| x.to_owned())
}
