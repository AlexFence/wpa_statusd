#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate bson;
extern crate config;
extern crate env_logger;
extern crate serde;
extern crate wpactrl;

mod commands;
mod daemon;
mod protocol;
mod status;

use daemon::Daemon;

fn main() {
    env_logger::init();
    let conf = daemon::get_config();
    let d = Daemon::new(conf.unwrap());
    d.run()
}
