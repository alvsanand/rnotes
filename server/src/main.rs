#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate crypto;
extern crate dotenv;
extern crate jwt;
#[macro_use]
extern crate rocket;
extern crate env_logger;
extern crate rnotes_core;
extern crate rocket_contrib;
extern crate serde_derive;

mod handlers;

use chrono::Local;
use dotenv::dotenv;
use env_logger::Builder;
use log::*;
use rnotes_core::BDPool;
use rocket::config::{Config, Environment};
use std::env;
use std::io::Write;

#[get("/")]
fn index() -> &'static str {
    "rnotes server!"
}

fn main() {
    Builder::new()
        .format(|buf, record| {
            let file = record
                .file()
                .map(|f| f.split("src/").last().unwrap_or(""))
                .unwrap_or("");
            let line = record.line().unwrap_or(0);
            writeln!(
                buf,
                "{} [{}] [{}.{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                file,
                line,
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    dotenv().ok();

    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT")
        .expect("ROCKET_PORT must be set")
        .parse::<u16>()
        .expect("ROCKET_PORT must be an integer");

    let config = Config::build(Environment::Staging)
        .address(host)
        .port(port)
        .finalize()
        .expect("Cannot configure Rocket");

    let rocket = rocket::custom(config)
        .manage(BDPool::new().expect("Cannot obtain BDPool"))
        .mount("/", routes![index])
        .mount(
            "/notes",
            routes![
                handlers::notes::all,
                handlers::notes::get,
                handlers::notes::create,
                handlers::notes::update,
                handlers::notes::delete,
            ],
        )
        .mount(
            "/categories",
            routes![handlers::categories::all, handlers::categories::get,],
        )
        .mount("/auth", routes![handlers::auth::login])
        .attach(handlers::catch_not_json());

    info!("Launching rocket[port={}]", port);
    let rocket_err = rocket.launch();

    error!("Error launching rocket: {:?}", rocket_err);
}
