use afire::Server;

use crate::App;
mod run;

pub fn attach(server: &mut Server<App>) {
    run::attach(server);
}