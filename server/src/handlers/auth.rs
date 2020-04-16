use crate::handlers::{status_error, ApiResponse, StatusError};
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};
use rnotes_core::models::api::auth::{LoginIn, LoginOut};
use rnotes_core::models::db::user::User as DBUser;
use rnotes_core::DBConn;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/login", format = "application/json", data = "<request>")]
pub fn login<'r>(
    request: Json<LoginIn>,
    connection: DBConn,
) -> Result<ApiResponse<LoginOut>, StatusError<'r>> {
    let header: Header = Default::default();

    let email = request.email.clone();
    let password = request.password.clone();
    DBUser::find_by_email_and_password(&connection, email, password)
        .map_err(|_| status_error(Status::NotFound, format!("Invalid credentials")))
        .and_then(|user| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64;

            let claims = Registered {
                sub: Some(user.id.to_string()),
                iat: Some(now),
                exp: Some(now + super::jwt::get_session_time()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token
                .signed(&super::jwt::get_secret_key(), Sha256::new())
                .map(|jwt_token| {
                    ApiResponse::ok(LoginOut {
                        jwt_token: jwt_token,
                    })
                })
                .map_err(|_| {
                    status_error(Status::InternalServerError, format!("Unknown server error."))
                })
        })
}
