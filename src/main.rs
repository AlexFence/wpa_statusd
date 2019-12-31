#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate bson;
extern crate config;
extern crate env_logger;
extern crate serde;
extern crate wpactrl;

#[cfg(feature = "systemd")]
extern crate systemd;

mod commands;
mod daemon;
mod network_list;
mod protocol;
mod status;

use daemon::Daemon;
use env_logger::Env;

const LOGO: &'static str = include_str!("../logo");

fn main() {
    print!("{}", LOGO);
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let conf = daemon::get_config();
    let d = Daemon::new(conf.unwrap());
    d.run()
}
