use std::time::SystemTime;

use crate::App;

use afire::{Method, Response, Server};
use rand::Rng;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/auth/redirect", move |app, _| {
        let state = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .map(|x| x as char)
            .collect::<String>();

        let epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        app.oauth_states.lock().push((state.to_owned(), epoch));

        let redirect = format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}/auth/complete&response_type=code&scope=profile&state={}",
            app.config.client_id,
            urlencoding::encode(&app.config.external_url),
            state
        );

        Response::new()
            .status(307)
            .header("Location", &redirect)
            .header("Cache-Control", "no-store")
            .text(format!("Redirecting to {}", redirect))
    });
}
