use crate::schema::*;

use diesel::prelude::*;
use diesel::result::QueryResult;

use std::cmp::Ordering;
use std::time::SystemTime;

#[derive(Debug, Eq, Queryable, AsChangeset)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Ord for Category {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Category {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Category {
    pub fn find_all(connection: &PgConnection) -> QueryResult<Vec<Category>> {
        use crate::schema::categories::dsl::*;
        categories.get_results::<Category>(connection)
    }

    pub fn find_by_id(connection: &PgConnection, _id: i32) -> QueryResult<Category> {
        use crate::schema::categories::dsl::*;
        categories.find(_id).get_result::<Category>(connection)
    }

    pub fn update(connection: &PgConnection, obj: &Category) -> QueryResult<Category> {
        use crate::schema::categories::dsl::*;
        diesel::update(categories.find(obj.id))
            .set((name.eq(obj.name.clone()), update_time.eq(SystemTime::now())))
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, _id: i32) -> QueryResult<usize> {
        use crate::schema::categories::dsl::*;
        diesel::delete(categories.find(_id)).execute(connection)
    }
}

#[derive(Debug, Copy, Clone, Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
}

impl<'a> NewCategory<'a> {
    pub fn new(name: &'a str) -> Self {
        NewCategory { name: name }
    }

    pub fn create(&self, connection: &PgConnection) -> QueryResult<Category> {
        use crate::schema::categories::dsl::*;

        diesel::insert_into(categories)
            .values(self)
            .get_result(connection)
    }
}
