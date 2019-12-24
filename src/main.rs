extern crate config;
extern crate wpactrl;
extern crate serde;
extern crate bson;
//extern crate serde_bytes;
//extern crate serde_cbor;

#[macro_use]
extern crate serde_derive;


pub mod daemon;
pub mod protocol;
mod commands;
mod status;

fn main() {
    let conf = daemon::get_config();
    let d = daemon::Daemon::new(conf.unwrap());
    d.run()
}
