use afire::Server;

use crate::App;
mod problem;
mod run;
mod self_info;

pub fn attach(server: &mut Server<App>) {
    run::attach(server);
    problem::attach(server);
    self_info::attach(server);
}
