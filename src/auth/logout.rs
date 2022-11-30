use crate::App;

use afire::{Method, Response, Server, SetCookie};

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/auth/logout", |_| {
        let cookie = SetCookie::new("session", "null").path("/").max_age(0);

        Response::new()
            .status(308)
            .header("Location", "/")
            .cookie(cookie)
    });
}
