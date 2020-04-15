#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
#[macro_use]
extern crate serde_derive;

pub mod models;
pub mod schema;
pub mod utils;

use diesel::prelude::*;
use diesel::r2d2::*;
use diesel::sql_query;

use dotenv::dotenv;
use log::*;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

use std::env;
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

const DEFAULT_DATABASE_MAX_CONNECTIONS: u32 = 5;

#[derive(Debug)]
pub enum BDPoolError {
    InvalidArguments,
    InternalError(String),
}

pub struct BDPool {
    pool: Pool,
    schema: Option<String>,
}

pub struct DBConn(pub PooledConnection);

impl BDPool {
    pub fn new() -> Result<BDPool, BDPoolError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").map_err(|_| BDPoolError::InvalidArguments)?;
        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .map_or(DEFAULT_DATABASE_MAX_CONNECTIONS, |x| {
                (x.parse::<u32>()).unwrap_or(DEFAULT_DATABASE_MAX_CONNECTIONS)
            });
        let schema = env::var("DATABASE_SCHEMA");

        info!("Creating DB pool to {:?}", database_url);
        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        Ok(BDPool {
            pool: Pool::builder()
                .max_size(database_max_connections)
                .test_on_check_out(true)
                .build(manager)
                .expect("DB pool"),
            schema: schema.map_or(None, |v| Some(v)),
        })
    }

    pub fn get(&self) -> Result<DBConn, BDPoolError> {
        info!("Get connection");
        let connection = self.pool.get().map_err(|err| BDPoolError::InternalError(err.to_string()))?;
        if let Some(schema) = self.schema.clone() {
            info!("Setting search_path to {:?}", schema);
            sql_query(format!("SET search_path TO {}", schema))
                .execute(&connection)
                .map_err(|err| warn!("{:?}", err))
                .ok();
        }

        Ok(DBConn(connection))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DBConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DBConn, Self::Error> {
        let pool = request.guard::<State<BDPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(conn),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DBConn {
    type Target = PooledConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
