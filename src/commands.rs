use std::collections::HashMap;

use crate::protocol::{ErrorCode, Message, Method, Request, ResponseResult};
use crate::status::Status;

pub fn handle_request(req: Request, config: HashMap<String, String>) -> Message {
    match req.method {
        Method::STATUS => handle_status(req, config),
        Method::PING => handle_ping(req),
        _ => Message::Error {
            id: Some(req.id),
            method: None,
            code: ErrorCode::MethodNotFound,
        },
    }
}

fn handle_status(req: Request, config: HashMap<String, String>) -> Message {
    if let Some(wpa_supplicant_socket) = config.get(&String::from("wpa_supplicant_socket")) {
        let mut wpa = wpactrl::WpaCtrl::new()
            .ctrl_path(wpa_supplicant_socket)
            .open()
            .unwrap();
        let status = wpa.request("STATUS").unwrap();
        debug!("wpa_supplicant status response:\n{}", status);
        let status_model = Status::parse(status);

        if status_model.is_some() {
            let result = ResponseResult::Status(status_model.unwrap());

            return Message::Response {
                id: req.id,
                method: req.method,
                result,
            };
        }
    } else {
        error!("wpa_supplicant_socket is not configured");
    }

    Message::Error {
        id: Some(req.id),
        method: Some(req.method),
        code: ErrorCode::InternalError,
    }
}

fn handle_ping(req: Request) -> Message {
    Message::Response {
        id: req.id,
        method: req.method,
        result: ResponseResult::Pong,
    }
}
