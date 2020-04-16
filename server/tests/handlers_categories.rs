extern crate dotenv;
extern crate rnotes_core;
extern crate rnotes_server;
extern crate rocket;
extern crate rocket_contrib;

use rnotes_core::models::api::category::CategoryOut;
use rnotes_core::BDPool;
use rnotes_server::handlers::categories::*;
use rnotes_server::handlers::jwt::JWTKey;
use rnotes_server::handlers::ApiResponse;
use std::panic;

#[test]
fn test_all() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let expected: Vec<CategoryOut> = vec![
        CategoryOut {
            id: 1,
            name: "cat_a".to_string(),
            create_time: "".to_string(),
            update_time: "".to_string(),
        },
        CategoryOut {
            id: 2,
            name: "cat_b".to_string(),
            create_time: "".to_string(),
            update_time: "".to_string(),
        },
    ];

    match all(JWTKey::new("1".to_string()), con) {
        Ok(ApiResponse { json, status }) => {
            assert_eq!(status.code, 200);

            let result = json.unwrap();
            assert_eq!(result.0, expected);
        }
        _ => panic!("Unexpected response"),
    }
}

#[test]
fn test_get_ok() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let expected = CategoryOut {
        id: 1,
        name: "cat_a".to_string(),
        create_time: "".to_string(),
        update_time: "".to_string(),
    };

    match get(JWTKey::new("1".to_string()), con, 1) {
        Ok(ApiResponse { json, status }) => {
            assert_eq!(status.code, 200);

            let result = json.unwrap();
            assert_eq!(result.0, expected);
        }
        _ => panic!("Unexpected response"),
    }
}

#[test]
fn test_get_ko() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    match get(JWTKey::new("1".to_string()), con, 999) {
        Err(response) => {
            assert_eq!(response.status().code, 404);
        }
        _ => panic!("Unexpected response"),
    }
}
