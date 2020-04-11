extern crate dotenv;
extern crate rnotes_core;

use rnotes_core::models::category::*;
use rnotes_core::utils::eq_no_ord;
use rnotes_core::BDPool;

use std::panic;
use std::time::SystemTime;

#[test]
fn test_category_find_all() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = Category::find_all(&con).unwrap();

    let expected: Vec<Category> = vec![
        Category {
            id: 1,
            name: "cat_a".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
        Category {
            id: 2,
            name: "cat_b".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
    ];

    assert_eq!(result.len() > 0, true);
    assert!(eq_no_ord(&result, &expected));
}

#[test]
fn test_category_find_by_id() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = Category::find_by_id(&con, 1).unwrap();

    let expected = Category {
        id: 1,
        name: "cat_a".to_string(),
        create_time: SystemTime::now(),
        update_time: SystemTime::now(),
    };

    assert_eq!(result, expected);
}

#[test]
fn test_category_insert() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_category = NewCategory::new("cat_test_category_insert");

    let result = new_category.create(&con).unwrap();

    let expected = Category {
        id: result.id,
        name: new_category.name.to_string(),
        create_time: SystemTime::now(),
        update_time: SystemTime::now(),
    };

    assert_eq!(result, expected);

    let result = Category::find_by_id(&con, result.id).unwrap();
    assert_eq!(result, expected);

    Category::delete(&con, result.id).unwrap();
}

#[test]
fn test_category_delete() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_category = NewCategory::new("cat_test_category_delete");

    let category = new_category.create(&con).unwrap();

    let result = Category::delete(&con, category.id).unwrap();

    assert_eq!(result, 1 as usize);

    let expected = Category::find_by_id(&con, category.id);
    assert_eq!(expected.is_err(), true);
}

#[test]
fn test_category_update() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_category = NewCategory::new("cat_test_category_update");

    let mut category = new_category.create(&con).unwrap();

    category.name = category.name + "_other";

    let result = Category::update(&con, &category).unwrap();

    assert_eq!(result.id, category.id);
    assert_eq!(result.name, category.name);
    assert_eq!(result.create_time, category.create_time);
    assert!(result.update_time > category.update_time);

    let expected = Category::find_by_id(&con, category.id).unwrap();
    assert_eq!(result, expected);

    Category::delete(&con, category.id).unwrap();
}
