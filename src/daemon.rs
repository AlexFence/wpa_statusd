use crate::protocol::{ErrorCode, Message, Method, Request};
use bson;
use bson::ordered::OrderedDocument;
use bson::Bson;
use config;
use std::collections::hash_map::HashMap;
use std::fs;
use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;

fn handle_client(mut stream: UnixStream) {
    let document = bson::decode_document(&mut stream);
    if !document.is_err() {
        let mut request_document: OrderedDocument = document.unwrap();
        debug!("incoming bson: {:#?}", request_document);

        // ensure that the request either doesn't contain a method,
        // which can and will be handled by serde and that it does
        // contain a valid method, otherwhise the request can't be
        // deserialized by serde and will trigger a MalformedRequest
        // error
        match request_document.get_str("method") {
            Ok(value) => {
                request_document.insert_bson(
                    String::from("method"),
                    Bson::String(String::from(Method::get_valid_method(value))),
                );
            }
            Err(_err) => {}
        }

        let bson_msg = bson::from_bson(bson::Bson::Document(request_document));
        if bson_msg.is_ok() {
            let msg: Message = bson_msg.unwrap();

            match msg {
                Message::Request { id, method, params } => {
                    let request = Request { id, method, params };
                    info!("request: {:#?}", request);

                    let response = crate::commands::handle_request(request);
                    info!("response: {:#?}", response);

                    return send_message(stream, response);
                }
                _ => {
                    let err = Message::Error {
                        id: None,
                        method: None,
                        code: ErrorCode::MalformedRequest,
                    };
                    info!("error: {:#?}", err);
                    return send_message(stream, err);
                }
            }
        }
    }

    let err = Message::Error {
        id: None,
        method: None,
        code: ErrorCode::MalformedRequest,
    };
    info!("error: {:#?}", err);
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

        Daemon { listener }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    // connection succeeded
                    thread::spawn(|| handle_client(stream));
                }
                Err(err) => {
                    // connection failed
                    break;
                }
            }
        }
    }
}

pub fn get_config() -> Result<HashMap<String, String>, config::ConfigError> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("/etc/wifi-chand.ini"))
        .unwrap();
    settings.deserialize::<HashMap<String, String>>()
}
