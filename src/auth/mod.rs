use afire::Server;

use crate::app::App;

mod complete;
mod logout;
mod redirect;

pub fn attach(server: &mut Server<App>) {
    redirect::attach(server);
    logout::attach(server);
    complete::attach(server);
}
