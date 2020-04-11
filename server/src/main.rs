#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate crypto;
extern crate dotenv;
extern crate jwt;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate rnotes_core;

mod handlers;
mod models;
mod utils;

use dotenv::dotenv;
use rnotes_core::BDPool;
use rocket::config::{Config, Environment};
use std::env;

#[get("/")]
fn index() -> &'static str {
    "rnotes server!"
}

fn main() {
    dotenv().ok();

    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT")
        .expect("ROCKET_PORT must be set")
        .parse::<u16>()
        .expect("ROCKET_PORT must be integer");

    let config = Config::build(Environment::Staging)
        .address(host)
        .port(port)
        .finalize()
        .unwrap();

    let rocket = rocket::custom(config)
        .manage(BDPool::new().expect("Cannot obtain BDPool"))
        .mount("/", routes![index])
        .mount(
            "/notes",
            routes![
                handlers::notes::all,
                handlers::notes::get,
                handlers::notes::post,
                handlers::notes::put,
                handlers::notes::delete,
            ],
        )
        .mount(
            "/categories",
            routes![handlers::category::all, handlers::category::get,],
        )
        .mount("/auth", routes![handlers::auth::login])
        .attach(handlers::catch_not_json());

    rocket.launch();
}
