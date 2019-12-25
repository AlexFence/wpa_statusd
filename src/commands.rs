use std::collections::HashMap;
use std::path::Path;

use crate::network_list;
use crate::protocol::{ErrorCode, Message, Method, Request, ResponseResult};
use crate::status::Status;

pub fn handle_request(req: Request, config: HashMap<String, String>) -> Message {
    match req.method {
        Method::STATUS => handle_status(req, config),
        Method::SUPPLICANT_RUNNING => handle_supplicant_running(req, config),
        Method::LIST_NETWORKS => handle_list_networks(req, config),
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
        // TODO handle this failing
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

fn handle_supplicant_running(req: Request, config: HashMap<String, String>) -> Message {
    if let Some(wpa_supplicant_socket) = config.get(&String::from("wpa_supplicant_socket")) {
        let running: bool = Path::new(wpa_supplicant_socket).exists();
        return Message::Response {
            id: req.id,
            method: req.method,
            result: ResponseResult::SupplicantRunning { running },
        };
    } else {
        error!("wpa_supplicant_socket is not configured");
    }

    Message::Error {
        id: Some(req.id),
        method: Some(req.method),
        code: ErrorCode::InternalError,
    }
}

fn handle_list_networks(req: Request, config: HashMap<String, String>) -> Message {
    if let Some(wpa_supplicant_socket) = config.get(&String::from("wpa_supplicant_socket")) {
        // TODO handle this failing
        let mut wpa = wpactrl::WpaCtrl::new()
            .ctrl_path(wpa_supplicant_socket)
            .open()
            .unwrap();
        let response = wpa.request("LIST_NETWORKS").unwrap();
        debug!("wpa_supplicant status response:\n{}", response);
        let networks = network_list::parse_list(response);
        let result = ResponseResult::Networks(networks);

        return Message::Response {
            id: req.id,
            method: req.method,
            result,
        };
    } else {
        error!("wpa_supplicant_socket is not configured");
    }

    Message::Error {
        id: Some(req.id),
        method: Some(req.method),
        code: ErrorCode::InternalError,
    }
}
