use crate::cmd::*;
use crate::http_client::HttpClient;
use crate::run::Runner;
use crate::CliOpt;
use crate::GenericError;
use dirs;
use figlet_rs::FIGfont;
use rustyline::completion::Completer;
use rustyline::config::CompletionType;
use rustyline::error::ReadlineError;
use rustyline::{hint::Hinter, Context};
use rustyline::{Cmd, Config, Editor, KeyPress};
use rustyline_derive::{Helper, Highlighter, Validator};
use shell_words;
use std::collections::HashSet;
use std::io::{self, Write};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Helper, Validator, Highlighter)]
struct CmdHelper {
    hints: HashSet<String>,
}

impl Completer for CmdHelper {
    type Candidate = std::string::String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut candidates: Vec<String> = self
            .hints
            .iter()
            .filter_map(|hint| {
                if pos > 0 && hint.starts_with(&line[..pos]) && !hint[pos..].contains(" ") {
                    Some(hint[pos..].to_owned())
                } else {
                    None
                }
            })
            .collect();
        candidates.sort();
        Ok((pos, candidates))
    }
}

impl Hinter for CmdHelper {
    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if pos < line.len() {
            return None;
        }

        let mut candidates: Vec<String> = self
            .hints
            .iter()
            .filter_map(|hint| {
                if pos > 0 && hint.starts_with(&line[..pos]) {
                    Some(hint[pos..].to_owned())
                } else {
                    None
                }
            })
            .collect();
        candidates.sort();
        candidates.first().map(|s| String::from(s))
    }
}

pub fn print_info() {
    let standard_font = FIGfont::standand().unwrap();
    println!("########################################################################");
    println!(
        "{}",
        standard_font.convert("rnotes_cli").expect("Unknown error")
    );
    println!("########################################################################");
}

pub async fn ui_loop(opt: CliOpt) -> Result<(), GenericError> {
    let ref history_file = format!(
        "{}/.rnotes_cli.history",
        dirs::home_dir().unwrap().to_str().unwrap()
    );

    let helper = CmdHelper { hints: cmd_hints() };

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .build();

    let mut rl: Editor<CmdHelper> = Editor::with_config(config);
    rl.set_helper(Some(helper));
    rl.bind_sequence(KeyPress::Tab, Cmd::Complete);
    rl.load_history(history_file).unwrap_or_default();

    let (in_tx, in_rx) = unbounded_channel();
    let (out_tx, mut out_rx) = unbounded_channel();

    let progress_chars = vec!["-", "\\", "|", "/"];

    tokio::spawn(runner_task(opt, in_rx, out_tx));

    loop {
        let readline = rl.readline("rnotes_cli>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match parse_line(&line) {
                    Ok(Command::Nothing) => (),
                    Ok(cmd) => {
                        in_tx.send(cmd)?;
                        let mut x = 0;
                        loop {
                            match out_rx.try_recv() {
                                Ok(msg) => {
                                    println!("{}", msg);
                                    break;
                                }
                                Err(_) => {
                                    print!("Executing command {}", progress_chars[x % 4]);
                                    io::stdout().flush()?;

                                    std::thread::sleep(std::time::Duration::from_millis(250));

                                    print!("\r");
                                    io::stdout().flush()?;

                                    x = x + 1;
                                }
                            }
                        }
                    }
                    Err(Error::Exit) => {
                        break;
                    }
                    Err(Error::Parse(error)) => {
                        println!("{}\n", error);
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    if rl.save_history(history_file).is_err() {
        println!("Error saving history file {}", history_file);
    }

    Ok(())
}

async fn runner_task(
    opt: CliOpt,
    mut rx: UnboundedReceiver<Command>,
    tx: UnboundedSender<String>,
) -> Result<(), GenericError> {
    let client = HttpClient::new();

    let mut runner = Runner::new(opt.server, client);

    while let Some(cmd) = rx.recv().await {
        tx.send(runner.run(cmd).await)?;
    }

    Ok(())
}

fn parse_line(buf: &String) -> Result<Command, Error> {
    let tokens = shell_words::split(&buf)
        .map_err(|_| Error::Parse(format!("Error parsing line: {}", buf.trim())))?;

    parse_command(tokens)
}
