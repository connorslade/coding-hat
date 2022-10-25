use afire::Server;

use crate::App;
mod run;
mod problem;

pub fn attach(server: &mut Server<App>) {
    run::attach(server);
    problem::attach(server);
}
