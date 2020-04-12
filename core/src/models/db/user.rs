use crate::schema::*;

use diesel::prelude::*;
use diesel::result::QueryResult;

use std::cmp::Ordering;
use std::time::SystemTime;

#[derive(Debug, Eq, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub password: String, //SHA256 of the password
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.password == other.password
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl User {
    pub fn find_all(connection: &PgConnection) -> QueryResult<Vec<User>> {
        use crate::schema::users::dsl::*;
        users.get_results::<User>(connection)
    }

    pub fn find_by_id(connection: &PgConnection, _id: i32) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users.find(_id).get_result::<User>(connection)
    }

    pub fn find_by_email_and_password(
        connection: &PgConnection,
        _email: String,
        _password: String,
    ) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users
            .filter(email.eq(_email))
            .filter(password.eq(_password))
            .get_result::<User>(connection)
    }

    pub fn update(connection: &PgConnection, obj: &User) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        diesel::update(users.find(obj.id))
            .set((
                email.eq(obj.email.clone()),
                name.eq(obj.name.clone()),
                password.eq(obj.password.clone()),
                update_time.eq(SystemTime::now()),
            ))
            .get_result(connection)
    }

    pub fn delete(connection: &PgConnection, _id: i32) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;
        diesel::delete(users.find(_id)).execute(connection)
    }
}

#[derive(Debug, Copy, Clone, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub password: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(email: &'a str, name: &'a str, password: &'a str) -> Self {
        NewUser {
            email: email,
            name: name,
            password: password,
        }
    }

    pub fn create(&self, connection: &PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(self)
            .get_result(connection)
    }
}
