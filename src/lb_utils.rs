use crate::lb_types::*;
use rocket::http::Status;
use rocket::serde::json::Json;

pub fn lb_respond<'a>(code: Status, message: &'a str) -> (Status, Json<APIResponse<'a>>) {
    (code, Json(APIResponse {
        code: code.code,
        status: None,
        message: message
    }))
}
pub fn lb_ok<'a>() -> (Status, Json<APIResponse<'a>>) {
    (Status::Ok, Json(APIResponse {
        code: 200,
        status: Some("ok"),
        message: ""
    }))
}