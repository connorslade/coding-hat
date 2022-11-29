use std::time::SystemTime;

use crate::{misc::json_err, App};

use afire::{Method, Response, Server, SetCookie};
use rand::Rng;
use rusqlite::params;
use serde_json::Value;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/auth/complete", move |app, req| {
        // Get Code from URI
        let code = match req.query.get("code") {
            Some(i) => i,
            _ => return json_err("No Auth Code Found"),
        };

        // Get and verify state
        let state = match req.query.get("state") {
            Some(i) => i,
            _ => return json_err("No State Found"),
        };

        let epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        {
            let mut os = app.oauth_states.lock();
            let real_state = match os.iter().position(|x| x.0 == state) {
                Some(i) => os.remove(i),
                None => return json_err("Invalid State"),
            };

            if epoch - real_state.1 >= 60 * 10 {
                return json_err("State Expired");
            }

            if real_state.0 != state {
                return json_err("State Dosent Match");
            }
        }

        // Get Access Token
        let resp = ureq::post("https://oauth2.googleapis.com/token")
            .timeout(app.config.req_duration)
            .send_form(&[
                ("grant_type", "authorization_code"),
                ("client_secret", &app.config.client_secret),
                ("client_id", &app.config.client_id),
                ("code", &urlencoding::decode(&code).unwrap()),
                (
                    "redirect_uri",
                    &format!("{}/auth/complete", app.config.external_url),
                ),
            ])
            .unwrap()
            .into_reader();

        // Parse Response and net Token
        let token = serde_json::from_reader::<_, Value>(resp).unwrap();
        let access_token = token
            .get("access_token")
            .and_then(|x| x.as_str())
            .expect("No Access Token!?");

        // Get User Info
        let user_raw = ureq::get("https://www.googleapis.com/oauth2/v1/userinfo")
            .set("Authorization", &format!("Bearer {}", access_token))
            .timeout(app.config.req_duration)
            .call()
            .unwrap()
            .into_reader();

        // Parse JSON
        let user = serde_json::from_reader::<_, Value>(user_raw).unwrap();
        let id = user.get("id").and_then(|x| x.as_str()).expect("No ID");
        let name = user.get("name").and_then(|x| x.as_str()).expect("No Name");
        let picture = user
            .get("picture")
            .and_then(|x| x.as_str())
            .expect("No Picture");

        dbg!(id, name, picture);
        app.db
            .lock()
            .execute(
                include_str!("../sql/upsert_login.sql"),
                params![id, name, picture, access_token],
            )
            .unwrap();

        // TODO: Garbage collect sessions?
        let session_token = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(15)
            .map(|x| x as char)
            .collect::<String>();

        app.db
            .lock()
            .execute(
                "INSERT INTO sessions (created, user_id, session_id) VALUES (?, ?, ?)",
                params![epoch, id, session_token],
            )
            .unwrap();

        let cookie = SetCookie::new("session", session_token)
            .path("/")
            .max_age(30 * 24 * 60 * 60);

        Response::new()
            .status(308)
            .header("Location", "/")
            .cookie(cookie)
    });
}
