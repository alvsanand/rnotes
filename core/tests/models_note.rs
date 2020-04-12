extern crate dotenv;
extern crate rnotes_core;

use rnotes_core::models::db::note::*;
use rnotes_core::utils::eq_no_ord;
use rnotes_core::BDPool;

use std::panic;
use std::time::SystemTime;

#[test]
fn test_note_find_all() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = Note::find_all(&con).unwrap();

    let expected: Vec<Note> = vec![
        Note {
            id: 1,
            user_id: 1,
            category_id: None,
            title: "note_a_user_a".to_string(),
            data: "some_text_note_a_user_a".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
        Note {
            id: 2,
            user_id: 1,
            category_id: Some(1),
            title: "note_b_user_a".to_string(),
            data: "some_text_note_a_user_a".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
        Note {
            id: 3,
            user_id: 1,
            category_id: Some(2),
            title: "note_c_user_c".to_string(),
            data: "some_text_note_c_user_c".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
        Note {
            id: 4,
            user_id: 2,
            category_id: None,
            title: "note_a_user_b".to_string(),
            data: "some_text_note_a_user_b".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
    ];

    assert_eq!(result.len() > 0, true);
    assert!(eq_no_ord(&result, &expected));
}

#[test]
fn test_note_find_by_id() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = Note::find_by_user_id(&con, 1).unwrap();

    let expected: Vec<Note> = vec![
        Note {
            id: 1,
            user_id: 1,
            category_id: None,
            title: "note_a_user_a".to_string(),
            data: "some_text_note_a_user_a".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
        Note {
            id: 2,
            user_id: 1,
            category_id: Some(1),
            title: "note_b_user_a".to_string(),
            data: "some_text_note_a_user_a".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
        Note {
            id: 3,
            user_id: 1,
            category_id: Some(2),
            title: "note_c_user_c".to_string(),
            data: "some_text_note_c_user_c".to_string(),
            create_time: SystemTime::now(),
            update_time: SystemTime::now(),
        },
    ];

    assert_eq!(result.len() > 0, true);
    assert!(eq_no_ord(&result, &expected));
}

#[test]
fn test_note_find_by_user_id() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = Note::find_by_id(&con, 1).unwrap();

    let expected = Note {
        id: 1,
        user_id: 1,
        category_id: None,
        title: "note_a_user_a".to_string(),
        data: "some_text_note_a_user_a".to_string(),
        create_time: SystemTime::now(),
        update_time: SystemTime::now(),
    };

    assert_eq!(result, Note::from(expected));
}

#[test]
fn test_notefind_by_id_and_user_id_ok() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = Note::find_by_id_and_user_id(&con, 1, 1).unwrap();

    let expected = Note {
        id: 1,
        user_id: 1,
        category_id: None,
        title: "note_a_user_a".to_string(),
        data: "some_text_note_a_user_a".to_string(),
        create_time: SystemTime::now(),
        update_time: SystemTime::now(),
    };

    assert_eq!(result, Note::from(expected));
}

#[test]
fn test_notefind_by_id_and_user_id_ko() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let result = Note::find_by_id_and_user_id(&con, 1, 2);

    assert!(result.is_err());
}

#[test]
fn test_note_insert() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_note = NewNote::new(
        1,
        Some(1),
        "note_test_test_note_insert",
        "some_text_note_test_note_insert",
    );

    let result = new_note.create(&con).unwrap();

    let expected = Note {
        id: result.id,
        user_id: new_note.user_id,
        category_id: new_note.category_id,
        title: new_note.title.to_string(),
        data: new_note.data.to_string(),
        create_time: SystemTime::now(),
        update_time: SystemTime::now(),
    };

    assert_eq!(result, expected);

    let result = Note::find_by_id(&con, result.id).unwrap();
    assert_eq!(result, expected);

    Note::delete(&con, result.id).unwrap();
}

#[test]
fn test_note_delete() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_note = NewNote::new(
        1,
        Some(1),
        "note_test_note_delete",
        "some_text_note_test_note_delete",
    );

    let note = new_note.create(&con).unwrap();

    let result = Note::delete(&con, note.id).unwrap();

    assert_eq!(result, 1 as usize);

    let expected = Note::find_by_id(&con, note.id);
    assert_eq!(expected.is_err(), true);
}

#[test]
fn test_note_update() {
    let pool = BDPool::new().unwrap();
    let con = pool.get().unwrap();

    let new_note = NewNote::new(
        1,
        Some(1),
        "note_test_note_update",
        "some_text_note_test_note_update",
    );

    let mut note = new_note.create(&con).unwrap();

    note.user_id = 2;
    note.category_id = Some(2);
    note.title = note.title + "_other";
    note.data = note.data + "_other";

    let result = Note::update(&con, &note).unwrap();

    assert_eq!(result.id, note.id);
    assert_eq!(result.user_id, note.user_id);
    assert_eq!(result.category_id, note.category_id);
    assert_eq!(result.title, note.title);
    assert_eq!(result.data, note.data);
    assert_eq!(result.create_time, note.create_time);
    assert!(result.update_time > note.update_time);

    let expected = Note::find_by_id(&con, note.id).unwrap();
    assert_eq!(result, expected);

    Note::delete(&con, note.id).unwrap();
}
