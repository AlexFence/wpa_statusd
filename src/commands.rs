use crate::protocol::{ErrorCode, Message, Method, Request, ResponseResult};
use crate::status::Status;

pub fn handle_request(req: Request) -> Message {
    match req.method {
        Method::STATUS => handle_status(req),
        Method::PING => handle_ping(req),
        _ => Message::Error {
            id: Some(req.id),
            method: None,
            code: ErrorCode::MethodNotFound,
        },
    }
}

fn handle_status(req: Request) -> Message {
    let mut wpa = wpactrl::WpaCtrl::new()
        .ctrl_path("/run/wpa_supplicant/wls1")
        .open()
        .unwrap();
    let status = wpa.request("STATUS").unwrap();
    let status_model = Status::parse(status);

    if status_model.is_some() {
        let result = ResponseResult::Status(status_model.unwrap());

        return Message::Response {
            id: req.id,
            method: req.method,
            result,
        };
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
