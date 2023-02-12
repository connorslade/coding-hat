use std::fs::{self, File};
use std::path::PathBuf;
use std::process;

use afire::{
    extension::{Logger, ServeStatic},
    Content, Middleware, Response, Server,
};

mod api;
mod app;
mod auth;
mod config;
mod r#const;
mod langs;
mod misc;
mod problem;
use app::App;

fn main() {
    // Make sure web dist has been built
    if !PathBuf::from("web/dist").exists() {
        println!("[-] Web dist not built!");
        // return;
    }
    r#const::init();

    // Create App
    let app = App::new();
    let _ = fs::create_dir(&app.config.tmp_folder);
    let workers = app.config.workers;

    // Init Server
    let mut server = Server::<App>::new(app.config.host.as_str(), app.config.port).state(app);
    ServeStatic::new("web/dist")
        .not_found(|_req, _dis| {
            Response::new()
                .stream(File::open("web/dist/index.html").expect("Webpage not built"))
                .content(Content::HTML)
        })
        .attach(&mut server);
    Logger::new().attach(&mut server);
    api::attach(&mut server);
    auth::attach(&mut server);

    // Setup Exit Handler
    let error_app = server.state.as_ref().unwrap().clone();
    ctrlc::set_handler(move || {
        error_app
            .db
            .lock()
            .pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .unwrap();
        process::exit(0);
    })
    .unwrap();

    // Start Server
    server.start_threaded(workers).unwrap()
}
