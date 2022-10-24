use std::fs;
use std::path::PathBuf;
use std::process;

use afire::{
    extension::{Logger, ServeStatic},
    Middleware, Server,
};

mod api;
mod app;
mod config;
use app::App;

fn main() {
    // Make sure web dist has been built
    if !PathBuf::from("web/dist").exists() {
        println!("[-] Web dist not built!");
        return;
    }

    // Create App
    let app = App::new();
    let _ = fs::create_dir(&app.config.tmp_folder);
    let workers = app.config.workers;

    // Init Server
    let mut server = Server::<App>::new(&app.config.host, app.config.port).state(app);
    ServeStatic::new("web/dist").attach(&mut server);
    Logger::new().attach(&mut server);
    api::attach(&mut server);

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
