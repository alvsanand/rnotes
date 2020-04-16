extern crate dotenv;
extern crate rnotes_core;

use rnotes_core::models::db::user::*;
use rnotes_core::utils::eq_no_ord;
use rnotes_core::BDPool;

use std::panic;
use std::time::SystemTime;

#[test]
fn test_pool() {
    let pool = BDPool::new().unwrap();
    pool.get().unwrap();
}

#[test]
fn test_user_find_all() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = User::find_all(&con).unwrap();

    let expected: Vec<User> = vec![
        User {
            id: 1,
            email: "user_a@email.com".to_string(),
            name: "user_a".to_string(),
            password: "1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B"
                .to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
        User {
            id: 2,
            email: "user_b@email.com".to_string(),
            name: "user_b".to_string(),
            password: "1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B"
                .to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
        User {
            id: 3,
            email: "user_c@email.com".to_string(),
            name: "user_c".to_string(),
            password: "1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B"
                .to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
    ];

    assert_eq!(result.len() > 0, true);
    assert!(eq_no_ord(&result, &expected));
}

#[test]
fn test_user_find_by_id() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = User::find_by_id(&con, 1).unwrap();

    let expected = User {
        id: 1,
        email: "user_a@email.com".to_string(),
        name: "user_a".to_string(),
        password: "1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B".to_string(),
        create_time: SystemTime::now(),
        update_time: SystemTime::now(),
    };

    assert_eq!(result, expected);
}

#[test]
fn test_user_find_by_email_and_password() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = User::find_by_email_and_password(
        &con,
        "user_a@email.com".to_string(),
        "1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B".to_string(),
    )
    .unwrap();

    let expected = User {
        id: 1,
        email: "user_a@email.com".to_string(),
        name: "user_a".to_string(),
        password: "1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B".to_string(),
        create_time: SystemTime::now(),
        update_time: SystemTime::now(),
    };

    assert_eq!(result, expected);
}

#[test]
fn test_user_insert() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_user = NewUser::new(
        "user_test_user_insert@email.com",
        "user_test_user_insert",
        "some_password",
    );

    let result = new_user.create(&con).unwrap();

    let expected = User {
        id: result.id,
        email: "user_test_user_insert@email.com".to_string(),
        name: new_user.name.to_string(),
        password: new_user.password.to_string(),
        create_time: SystemTime::now(),
        update_time: SystemTime::now(),
    };

    assert_eq!(expected, result);

    let result = User::find_by_id(&con, result.id).unwrap();
    assert_eq!(expected, result);

    User::delete(&con, result.id).unwrap();
}

#[test]
fn test_user_delete() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_user = NewUser::new(
        "user_test_user_delete@email.com",
        "user_test_user_delete",
        "some_password",
    );

    let user = new_user.create(&con).unwrap();

    let result = User::delete(&con, user.id).unwrap();

    assert_eq!(result, 1 as usize);

    let expected = User::find_by_id(&con, user.id);
    assert_eq!(expected.is_err(), true);
}

#[test]
fn test_user_update() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_user = NewUser::new(
        "user_test_user_update@email.com",
        "user_test_user_update",
        "some_password",
    );

    let mut user = new_user.create(&con).unwrap();

    user.email = user.email + "_other";
    user.name = user.name + "_other";
    user.password = user.password + "_other";

    let result = User::update(&con, &user).unwrap();

    assert_eq!(result.id, user.id);
    assert_eq!(result.email, user.email);
    assert_eq!(result.name, user.name);
    assert_eq!(result.password, user.password);
    assert_eq!(result.create_time, user.create_time);
    assert!(result.update_time > user.update_time);

    let expected = User::find_by_id(&con, user.id).unwrap();
    assert_eq!(result, expected);

    User::delete(&con, user.id).unwrap();
}
