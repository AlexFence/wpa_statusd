use std::collections::hash_map::HashMap;
use std::fs;
use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;

use bson;
use bson::ordered::OrderedDocument;
use bson::Bson;
use config;

use crate::protocol::{ErrorCode, Message, Method, Request};

#[derive(Debug)]
pub struct Daemon {
    config: HashMap<String, String>,
    listener: UnixListener,
}

impl Daemon {
    fn startup(config: &HashMap<String, String>) -> UnixListener {
        let mut socket_path = "/tmp/wpa_statusd.sok";
        let configured_socket = config.get(&String::from("socket"));

        if configured_socket.is_some() {
            socket_path = configured_socket.unwrap();
        }

        let socket = Path::new(socket_path);

        if socket.exists() {
            let result = fs::remove_file(socket);

            if result.is_err() {
                error!("socket already exists: {}", socket_path);
                std::process::exit(1);
            }
        }

        let listener = UnixListener::bind(socket_path).unwrap();
        info!("bound socket to: {}", socket_path);
        listener
    }

    pub fn new(config: HashMap<String, String>) -> Self {
        let listener = Self::startup(&config);

        Daemon { listener, config }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    // connection succeeded
                    let map: HashMap<String, String> = self.config.clone();
                    thread::spawn(|| Daemon::handle_client(stream, map));
                }
                Err(err) => {
                    // connection failed
                    break;
                }
            }
        }
    }

    fn handle_client(mut stream: UnixStream, config: HashMap<String, String>) {
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

                        let response = crate::commands::handle_request(request, config);
                        info!("response: {:#?}", response);

                        return Daemon::send_message(stream, response);
                    }
                    _ => {
                        let err = Message::Error {
                            id: None,
                            method: None,
                            code: ErrorCode::MalformedRequest,
                        };
                        info!("error: {:#?}", err);
                        return Daemon::send_message(stream, err);
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
        return Daemon::send_message(stream, err);
    }

    fn send_message(mut stream: UnixStream, msg: Message) {
        let bson_data = bson::to_bson(&msg);

        if let bson::Bson::Document(document) = bson_data.unwrap() {
            let mut buf = Vec::new();
            bson::encode_document(&mut buf, &document);
            stream.write_all(&buf);
        }
    }
}

pub fn get_config() -> Result<HashMap<String, String>, config::ConfigError> {
    let mut settings = config::Config::default();
    let result = settings.merge(config::File::with_name("/etc/wpa_statusd.ini"));

    if result.is_err() {
        warn!("no config file found. (/etc/wpa_statusd.ini)");
    }

    // TODO why do we convert that to a hashmap???
    // seems dumb
    settings.deserialize::<HashMap<String, String>>()
}
