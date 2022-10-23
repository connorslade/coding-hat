use std::path::PathBuf;

use afire::{extension::ServeStatic, Middleware, Server};

fn main() {
    if !PathBuf::from("web/dist").exists() {
        println!("[-] Web dist not built!");
        return;
    }

    let mut server = Server::<()>::new("localhost", 8080);
    ServeStatic::new("web/dist").attach(&mut server);

    server.start().unwrap()
}
