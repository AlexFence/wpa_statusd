use std::io::prelude::*;
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;
use std::fs;
use std::thread;
use std::collections::hash_map::HashMap;
use config;
use bson;
use crate::protocol::{Message, ErrorCode, ResponseResult};


fn handle_client(mut stream: UnixStream) {
    let document = bson::decode_document(&mut stream);
    if !document.is_err() {
        let bson_msg = bson::from_bson(bson::Bson::Document(document.unwrap()));
        
        if true  { //bson_msg.is_ok() {
            let msg: Message = bson_msg.unwrap();
            
            println!("{:#?}", msg);
            match msg {
                Message::Request { id, method, .. } => {
                    let resp = Message::Response {
                        id, 
                        method, 
                        result: ResponseResult::Status { contected: true }  
                    };

                    println!("{:#?}", resp);
                    return send_message(stream, resp);
                },
                _ => {
                    let err = Message::Error {id: None, method: None, code: ErrorCode::MalformedRequest };
                    return send_message(stream, err);
                }
            }
        }
    }
    
    let err = Message::Error {id: None, method: None, code: ErrorCode::MalformedRequest };
    println!("{:#?}", err);
    return send_message(stream, err);
}

fn send_message(mut stream: UnixStream, msg: Message) {
    let bson_data = bson::to_bson(&msg);

    if let bson::Bson::Document(document) = bson_data.unwrap() {
        let mut buf = Vec::new();
        bson::encode_document(&mut buf, &document);
        stream.write_all(&buf);
    }
}

#[derive(Debug)]
pub struct Daemon {
    listener: UnixListener,
}

impl Daemon {
    fn startup() -> UnixListener {
        let old = Path::new("/tmp/wifi-chan.sok");

        if old.exists() {
            fs::remove_file(old);
        }

        UnixListener::bind("/tmp/wifi-chan.sok").unwrap()
    }

    pub fn new(config: HashMap<String, String>) -> Self {
        let listener = Self::startup();

        Daemon {listener}
    }

    pub fn run(&self){
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    /* connection succeeded */
                    thread::spawn(|| handle_client(stream));
                }
                Err(err) => {
                    /* connection failed */
                    break;
                }
            }
        }
    }
}

pub fn get_config() -> Result<HashMap<String, String>, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("/etc/wifi-chand.ini")).unwrap();
    settings.deserialize::<HashMap<String, String>>()
}
