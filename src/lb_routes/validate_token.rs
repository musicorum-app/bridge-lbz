use rocket::serde::json::Json;
use crate::lb_types::ValidateToken;

#[rocket::get("/validate-token")]
pub fn validate_token() -> Json<ValidateToken<'static>> {
    Json(ValidateToken {
        code: 200,
        user_name: "2",
        message: "Token valid.",
        valid: true
    })
}