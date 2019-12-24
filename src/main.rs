extern crate bson;
extern crate config;
extern crate env_logger;
extern crate serde;
extern crate wpactrl;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod commands;
pub mod daemon;
pub mod protocol;
mod status;

fn main() {
    env_logger::init();
    let conf = daemon::get_config();
    let d = daemon::Daemon::new(conf.unwrap());
    d.run()
}
