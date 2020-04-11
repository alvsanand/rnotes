use crate::schema::*;

use diesel::prelude::*;
use diesel::result::QueryResult;

use std::cmp::Ordering;
use std::time::SystemTime;

#[derive(Debug, Eq, Queryable, AsChangeset)]
#[table_name = "notes"]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub category_id: Option<i32>,
    pub title: String,
    pub data: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.user_id == other.user_id
            && self.category_id == other.category_id
            && self.title == other.title
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Note {
    pub fn find_all(connection: &PgConnection) -> QueryResult<Vec<Note>> {
        use crate::schema::notes::dsl::*;
        notes.get_results::<Note>(connection)
    }

    pub fn find_by_id(connection: &PgConnection, _id: i32) -> QueryResult<Note> {
        use crate::schema::notes::dsl::*;
        notes.find(_id).get_result::<Note>(connection)
    }

    pub fn find_by_user_id(connection: &PgConnection, _user_id: i32) -> QueryResult<Vec<Note>> {
        use crate::schema::notes::dsl::*;
        notes.filter(user_id.eq(_user_id)).load::<Note>(connection)
    }

    pub fn find_by_id_and_user_id(
        connection: &PgConnection,
        _id: i32,
        _user_id: i32,
    ) -> QueryResult<Note> {
        use crate::schema::notes::dsl::*;
        notes
            .filter(id.eq(_id))
            .filter(user_id.eq(_user_id))
            .first::<Note>(connection)
    }

    pub fn update(connection: &PgConnection, obj: &Note) -> QueryResult<Note> {
        use crate::schema::notes::dsl::*;
        diesel::update(notes.find(obj.id))
            .set((
                user_id.eq(obj.user_id),
                category_id.eq(obj.category_id),
                title.eq(obj.title.clone()),
                data.eq(obj.data.clone()),
                update_time.eq(SystemTime::now()),
            ))
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, _id: i32) -> QueryResult<usize> {
        use crate::schema::notes::dsl::*;
        diesel::delete(notes.find(_id)).execute(connection)
    }
}

#[derive(Debug, Copy, Clone, Insertable)]
#[table_name = "notes"]
pub struct NewNote<'a> {
    pub user_id: i32,
    pub category_id: Option<i32>,
    pub title: &'a str,
    pub data: &'a str,
}

impl<'a> NewNote<'a> {
    pub fn new(user_id: i32, category_id: Option<i32>, title: &'a str, data: &'a str) -> Self {
        NewNote {
            user_id: user_id,
            category_id: category_id,
            title: title,
            data: data,
        }
    }

    pub fn create(&self, connection: &PgConnection) -> QueryResult<Note> {
        use crate::schema::notes::dsl::*;

        diesel::insert_into(notes)
            .values(self)
            .get_result(connection)
    }
}
