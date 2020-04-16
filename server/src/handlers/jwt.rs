use crypto::sha2::Sha256;
use dotenv::dotenv;
use hyper::header::{self, Authorization, Bearer};
use jwt::{Header, Registered, Token};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use std::env;

pub struct JWTKey {
    pub id_user: i32,
}

impl JWTKey {
    pub fn new(id_user: String) -> JWTKey {
        JWTKey {
            id_user: id_user.parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum JwtError {
    Missing,
    Invalid,
}

pub fn read_token(key: &str) -> Result<Registered, String> {
    let token =
        Token::<Header, Registered>::parse(key).map_err(|_| "Unable to parse key".to_string())?;
    if token.verify(&get_secret_key(), Sha256::new()) {
        Ok(token.claims)
    } else {
        Err("Token not valid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for JWTKey {
    type Error = JwtError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        if let Some(raw) = request.headers().get_one("Authorization") {
            let header: Result<Authorization<Bearer>, _> =
                header::Header::parse_header(&[raw.as_bytes().to_vec()]);
            match header {
                Ok(header) => match read_token(&header.0.token) {
                    Ok(claim) => {
                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as u64;

                        if claim.iat.is_some()
                            && claim.iat.unwrap() <= now
                            && claim.exp.is_some()
                            && claim.exp.unwrap() > now
                        {
                            Outcome::Success(JWTKey::new(claim.sub.unwrap()))
                        } else {
                            Outcome::Failure((Status::Unauthorized, JwtError::Invalid))
                        }
                    }
                    Err(_) => Outcome::Failure((Status::Unauthorized, JwtError::Invalid)),
                },
                _ => Outcome::Failure((Status::Unauthorized, JwtError::Invalid)),
            }
        } else {
            Outcome::Failure((Status::Unauthorized, JwtError::Missing))
        }
    }
}

pub fn get_secret_key() -> Vec<u8> {
    dotenv().ok();

    env::var("JWT_SECRET_KEY")
        .expect("JWT_SECRET_KEY must be set")
        .into_bytes()
}

pub fn get_session_time() -> u64 {
    dotenv().ok();

    env::var("JWT_SESSION_TIME")
        .expect("JWT_SESSION_TIME must be set")
        .parse::<u64>()
        .expect("JWT_SESSION_TIME must be an number")
}

#[test]
fn test_read_token() {
    let token = "eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOm51bGwsInN1YiI6IjEiLCJhdWQiOm51bGwsImV4cCI6MTU4NzAyNzkyNSwibmJmIjpudWxsLCJpYXQiOjE1ODcwMjQzMjUsImp0aSI6bnVsbH0.AN8RMyD8pHcMOft+LxXsLu1cTOKiEWk2mC6YYu6pDKw";

    let result = read_token(token);

    assert!(result.is_ok());
}
