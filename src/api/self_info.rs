use std::time::{SystemTime, UNIX_EPOCH};

use afire::{Content, Method, Response, Server};
use rusqlite::Error;
use serde_json::json;

use crate::{
    misc::{get_cookie, json_err},
    r#const::VALID_SESSION_LENGTH,
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/api/self_info", |app, req| {
        // Get session from requesr
        let session_id = match get_cookie(&req, "session") {
            Some(i) => i.value,
            None => {
                return Response::new()
                    .text(json!({ "error": "No Session!?" }))
                    .content(Content::JSON)
            }
        };

        // Querry database
        let (id, name, avatar, new, session_epoch) = match app.db.lock().query_row(
            include_str!("../sql/querry_self.sql"),
            [&session_id],
            |req| {
                Ok((
                    req.get::<_, String>(0)?,
                    req.get::<_, String>(1)?,
                    req.get::<_, String>(2)?,
                    req.get::<_, u8>(3)?,
                    req.get::<_, u64>(4)?,
                ))
            },
        ) {
            Ok(i) => i,
            Err(Error::QueryReturnedNoRows) => return json_err("Session not found"),
            Err(e) => panic!("{:?}", e),
        };

        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if epoch - session_epoch > VALID_SESSION_LENGTH {
            return json_err("Session expired");
        }

        app.db
            .lock()
            .execute(include_str!("../sql/update_not_new.sql"), [session_id])
            .unwrap();

        Response::new()
            .text(json!({
                "id": id,
                "name": name,
                "avatar": avatar,
                "new": new == 1,
            }))
            .content(Content::JSON)
    });
}
