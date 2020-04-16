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

pub mod handlers;