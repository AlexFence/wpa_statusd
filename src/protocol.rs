use crate::network_list::Network;
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
    LIST_NETWORKS,
    SUPPLICANT_RUNNING,
    PING,
    NOTFOUND,
}

impl Method {
    pub fn not_found() -> Method {
        Method::NOTFOUND
    }

    pub fn get_valid_method(string: &str) -> &str {
        match string {
            "STATUS" => "STATUS",
            "status" => "STATUS",
            "PING" => "PING",
            "ping" => "PING",
            "LIST_NETWORKS" => "LIST_NETWORKS",
            "list_networks" => "LIST_NETWORKS",
            "SUPPLICANT_RUNNING" => "SUPPLICANT_RUNNING",
            "supplicant_running" => "SUPPLICANT_RUNNING",
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
    SupplicantRunning { running: bool },
    Networks(Vec<Network>),
    Pong,
}

#[derive(Debug)]
pub struct Request {
    pub id: String,
    pub method: Method,
    pub params: Option<RequestParams>,
}
