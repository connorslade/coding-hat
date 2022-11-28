use afire::Server;

use crate::app::App;

mod complete;
mod redirect;

pub fn attach(server: &mut Server<App>) {
    redirect::attach(server);
    complete::attach(server);
}
