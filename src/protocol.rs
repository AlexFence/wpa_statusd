use crate::status::Status;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    Request {
        id: String,
        #[serde(default = "Method::not_found")]
        method: Method,
        params: Option<RequestParams>,
    },
    Response {
        id: String,
        method: Method,
        result: ResponseResult,
    },
    Error {
        id: Option<String>,
        method: Option<Method>,
        code: ErrorCode,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    STATUS,
    PING,
    NOTFOUND,
}

impl Method {
    pub fn not_found() -> Method {
        Method::NOTFOUND
    }

    pub fn get_from_string(string: &str) -> Method {
        match string {
            "STATUS" => Method::STATUS,
            "PING" => Method::PING,
            _ => Method::NOTFOUND,
        }
    }

    pub fn get_valid_method(string: &str) -> &str {
        match string {
            "STATUS" => "STATUS",
            "status" => "STATUS",
            "PING" => "PING",
            "ping" => "PING",
            _ => "NOTFOUND",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorCode {
    MethodNotFound,
    MalformedRequest,
    InternalError,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestParams {
    X { x: i32 },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseResult {
    Status(Status),
    Pong,
}

pub struct Request {
    pub id: String,
    pub method: Method,
    pub params: Option<RequestParams>,
}
