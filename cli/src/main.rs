#![feature(proc_macro_hygiene, decl_macro)]
extern crate clap;
extern crate dirs;
extern crate dotenv;
extern crate hyper;
extern crate rnotes_core;
extern crate rustyline;
extern crate rustyline_derive;
extern crate serde_derive;
extern crate sha2;
extern crate structopt;
extern crate tokio;

mod cmd;
mod http_client;
mod run;
mod ui;

use structopt::StructOpt;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

#[derive(StructOpt, Debug)]
#[structopt(name = "rnotes_cli")]
/// rnotes command line client
pub struct CLIOpt {
    /// http://{HOSTNAME}:{PORT} of rnotes server
    #[structopt(name = "SERVER", default_value = "http://localhost:8080")]
    server: String,
}

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let opt = CLIOpt::from_args();

    ui::print_info();

    ui::ui_loop(opt).await?;

    Ok(())
}
