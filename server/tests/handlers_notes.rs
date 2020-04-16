extern crate dotenv;
extern crate rnotes_core;
extern crate rnotes_server;
extern crate rocket;
extern crate rocket_contrib;

use rnotes_core::models::api::note::{NoteIn, NoteOut};
use rnotes_core::BDPool;
use rnotes_server::handlers::jwt::JWTKey;
use rnotes_server::handlers::notes::*;
use rnotes_server::handlers::ApiResponse;
use rocket_contrib::json::Json;
use std::panic;

#[test]
fn test_all() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let expected: Vec<NoteOut> = vec![
        NoteOut {
            id: 1,
            category_id: None,
            title: "note_a_user_a".to_string(),
            data: "some_text_note_a_user_a".to_string(),
            create_time: "".to_string(),
            update_time: "".to_string(),
        },
        NoteOut {
            id: 2,
            category_id: Some(1),
            title: "note_b_user_a".to_string(),
            data: "some_text_note_a_user_a".to_string(),
            create_time: "".to_string(),
            update_time: "".to_string(),
        },
        NoteOut {
            id: 3,
            category_id: Some(2),
            title: "note_c_user_c".to_string(),
            data: "some_text_note_c_user_c".to_string(),
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

    let expected = NoteOut {
        id: 1,
        category_id: None,
        title: "note_a_user_a".to_string(),
        data: "some_text_note_a_user_a".to_string(),
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
fn test_get_ko_note_id() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    match get(JWTKey::new("1".to_string()), con, 999) {
        Err(response) => {
            assert_eq!(response.status().code, 404);
        }
        _ => panic!("Unexpected response"),
    }
}

#[test]
fn test_get_ko_user_id() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    match get(JWTKey::new("999".to_string()), con, 1) {
        Err(response) => {
            assert_eq!(response.status().code, 404);
        }
        _ => panic!("Unexpected response"),
    }
}

#[test]
fn test_create() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let input = NoteIn {
        category_id: Some(1),
        title: "new_note".to_string(),
        data: "some_text_new_note".to_string(),
    };

    let mut expected = NoteOut {
        id: 1,
        category_id: Some(1),
        title: "new_note".to_string(),
        data: "some_text_new_note".to_string(),
        create_time: "".to_string(),
        update_time: "".to_string(),
    };

    match create(JWTKey::new("1".to_string()), con, Json(input)) {
        Ok(ApiResponse { json, status }) => {
            assert_eq!(status.code, 201);

            let result = json.unwrap();

            expected.id = result.0.id;

            assert_eq!(result.0, expected);

            delete(
                JWTKey::new("1".to_string()),
                pool.get().unwrap(),
                expected.id,
            )
            .expect("Unexpected error");
        }
        _ => panic!("Unexpected response"),
    }
}

#[test]
fn test_update() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let input = NoteIn {
        category_id: Some(1),
        title: "new_note".to_string(),
        data: "some_text_new_note".to_string(),
    };

    let updated_input = NoteIn {
        category_id: Some(1),
        title: "other_note".to_string(),
        data: "other_text_new_note".to_string(),
    };

    let mut expected = NoteOut {
        id: 1,
        category_id: Some(1),
        title: "other_note".to_string(),
        data: "other_text_new_note".to_string(),
        create_time: "".to_string(),
        update_time: "".to_string(),
    };

    match create(JWTKey::new("1".to_string()), con, Json(input)) {
        Ok(ApiResponse { json, status }) => {
            assert_eq!(status.code, 201);

            let result = json.unwrap();

            let id = result.0.id;
            expected.id = id;

            match update(
                JWTKey::new("1".to_string()),
                pool.get().unwrap(),
                id,
                Json(updated_input),
            ) {
                Ok(ApiResponse { json, status }) => {
                    assert_eq!(status.code, 201);
                    let result = json.unwrap();
                    assert_eq!(result.0, expected);

                    delete(
                        JWTKey::new("1".to_string()),
                        pool.get().unwrap(),
                        expected.id,
                    )
                    .expect("Unexpected error");
                }
                _ => panic!("Unexpected response"),
            }
        }
        _ => panic!("Unexpected response"),
    }
}
