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
mod parse;

pub fn print_status() {
    let mut wpa = wpactrl::WpaCtrl::new().ctrl_path("/run/wpa_supplicant/wls1").open().unwrap();
    println!("{}", wpa.request("STATUS").unwrap());
}

fn main() {
    print_status();
    let conf = daemon::get_config();
    let d = daemon::Daemon::new(conf.unwrap());
    d.run()
}
