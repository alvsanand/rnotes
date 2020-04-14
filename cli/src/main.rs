#![feature(proc_macro_hygiene, decl_macro)]
extern crate clap;
extern crate dirs;
extern crate dotenv;
extern crate rnotes_core;
extern crate rocket;
extern crate rocket_contrib;
extern crate rustyline;
extern crate rustyline_derive;
extern crate serde_derive;
extern crate structopt;

mod cmd;
mod run;
mod ui;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rnotes_cli")]
/// rnotes command line client
struct CLIOpt {
    /// {HOSTNAME}:{PORT} of rnotes server
    #[structopt(name = "SERVER", default_value = "localhost:8080")]
    server: String,
}

fn main() {
    let _opt = CLIOpt::from_args();

    ui::ui_loop();
}
