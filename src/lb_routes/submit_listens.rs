use rocket::serde::json::Json;
use rocket::http::Status;
use crate::lb_types::{SubmitListens, APIResponse};
use crate::lb_utils::{lb_respond, lb_ok};

#[rocket::post("/submit-listens", format="json", data = "<data>")]
pub fn submit_listens(data: Json<SubmitListens<'_>>) -> (Status, Json<APIResponse<'static>>) {
    let sl = data.into_inner();
    if sl.payload.len() == 0 {
        return lb_respond(Status::BadRequest, "No listens provided.");
    }
    if (sl.listen_type == "playing_now" || sl.listen_type == "single") && sl.payload.len() > 1 {
        return lb_respond(Status::BadRequest, "A playing_now or single listen type must have exactly one listen payload.");
    }
    
    println!("{:?}", sl.payload[0]);

    lb_ok()
}