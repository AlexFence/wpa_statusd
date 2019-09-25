//use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    Request { id: String, method: Method, params: Option<RequestParams> },
    Response { id: String, method: Method, result: ResponseResult },
    Error { id: Option<String>, method: Option<Method>, code: ErrorCode },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    STATUS
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorCode {
    MethodNotFound,
    MalformedRequest
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestParams {
    X { x: i32 }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseResult {
    Status { contected: bool }
}
