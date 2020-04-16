extern crate dotenv;
extern crate rnotes_core;
extern crate rnotes_server;
extern crate rocket;
extern crate rocket_contrib;

use rnotes_core::models::api::auth::*;
use rnotes_core::BDPool;
use rnotes_server::handlers::auth::login;
use rnotes_server::handlers::ApiResponse;
use std::panic;
use rocket_contrib::json::Json;

#[test]
fn test_login_ok() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let login_in = LoginIn {
        email: "user_a@email.com".to_string(),
        password: "1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B".to_string(),
    };

    match login(Json(login_in), con) {
        Ok(ApiResponse { json, status }) => {
            assert_eq!(status.code, 200);
            assert!(json.is_some() && !json.unwrap().jwt_token.is_empty());
        }
        _ => panic!("Unexpected response"),
    }
}

#[test]
fn test_login_ko() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let login_in = LoginIn {
        email: "user_a@email.com".to_string(),
        password: "bad_password".to_string(),
    };

    match login(Json(login_in), con) {
        Err(response) => {
            assert_eq!(response.status().code, 404);
        }
        _ => panic!("Unexpected response"),
    }
}
