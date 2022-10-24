use afire::{Method, Server};

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/run", |app, req| {
        //
        todo!()
    });
}
