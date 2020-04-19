extern crate rnotes_cli;

use rnotes_cli::{ui, CliOpt, GenericError};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let opt = CliOpt::from_args();

    ui::print_info();

    ui::ui_loop(opt).await?;

    Ok(())
}
