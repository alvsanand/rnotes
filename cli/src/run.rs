

use crate::cmd::*;

pub fn run_command(cmd: Command) {
    match cmd {
        Command::Help(message) => {
            println!("{}", message);
        },
        other => {
            println!("Received {:?}", other);
        }
    } 
}