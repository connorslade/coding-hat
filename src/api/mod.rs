use afire::Server;

use crate::App;
mod problem;
mod run;

pub fn attach(server: &mut Server<App>) {
    run::attach(server);
    problem::attach(server);
}
