use rocket::http::Status;
use rocket_contrib::json::Json;

use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};

use rnotes_core::models::user::User as DBUser;
use rnotes_core::DBConn;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    jwt_token: String,
}

#[post("/login", data = "<request>")]
pub fn login<'r>(
    request: Json<LoginRequest>,
    connection: DBConn,
) -> Result<Json<LoginResponse>, Status> {
    let header: Header = Default::default();

    let email = request.email.clone();
    let password = request.password.clone();
    match DBUser::find_by_email_and_password(&connection, email, password) {
        Ok(user) => {
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
                    Json(LoginResponse {
                        jwt_token: jwt_token,
                    })
                })
                .map_err(|_| Status::InternalServerError)
        }
        _ => Err(Status::NotFound),
    }
}
