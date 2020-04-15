use clap::{App, AppSettings};
use rnotes_core::models::api;
use std::collections::HashSet;
use structopt::StructOpt;

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Exit,
}

#[derive(Debug)]
pub enum Command {
    Nothing,
    Help(String),
    Auth(AuthCommand),
    Categories(CategoriesCommand),
    Notes(NotesCommand),
}

#[derive(Debug)]
pub enum AuthCommand {
    Login(api::auth::LoginIn),
}

#[derive(Debug)]
pub enum CategoriesCommand {
    All,
    Get(i32),
}

#[derive(Debug)]
pub enum NotesCommand {
    All,
    Get(i32),
    Create(api::note::NoteIn),
    Update(i32, api::note::NoteIn),
    Delete(i32),
}

pub fn cmd_hints() -> HashSet<String> {
    let mut set = HashSet::new();
    set.insert(String::from("auth"));
    set.insert(String::from("auth login"));
    set.insert(String::from("categories"));
    set.insert(String::from("categories all"));
    set.insert(String::from("categories get"));
    set.insert(String::from("notes"));
    set.insert(String::from("notes all"));
    set.insert(String::from("notes get"));
    set.insert(String::from("notes create"));
    set.insert(String::from("notes update"));
    set.insert(String::from("notes delete"));

    for v in set.clone().into_iter() {
        set.insert(format!("help {}", v));
    }

    set.insert(String::from("help"));
    set.insert(String::from("exit"));

    set
}

fn clean_help(help_str: String, cmd: Option<String>, srv: Option<String>) -> String {
    let tmp = if let Some(idx) = help_str.find("\n") {
        help_str.split_at(idx).1.to_string()
    } else {
        help_str.clone()
    };
    if cmd.is_some() && srv.is_some() {
        tmp.replace(
            "_cmd",
            &format!("{} {}", cmd.unwrap(), srv.clone().unwrap()),
        )
        .replace("SUBCOMMAND", "SERVICE")
        .trim()
        .to_string()
    } else {
        if cmd.is_some() {
            tmp.replace("_cmd <SUBCOMMAND>", &format!("{} <SERVICE>", cmd.unwrap()))
                .trim()
                .to_string()
        } else {
            tmp.replace("_cmd <SUBCOMMAND>", "<SERVICE>")
                .replace("SUBCOMMAND", "SERVICE")
                .trim()
                .to_string()
        }
    }
}

fn get_help(clap: &App, command: Option<String>, service: Option<String>) -> String {
    use std::io::{Cursor, Read, Seek, SeekFrom};
    use std::str::from_utf8;

    let mut buf = Cursor::new(Vec::new());

    clap.write_help(&mut buf).unwrap();

    let mut help_messages: Vec<u8> = Vec::new();
    buf.seek(SeekFrom::Start(0)).unwrap();
    buf.read_to_end(&mut help_messages).unwrap();

    let raw_help = from_utf8(&help_messages).unwrap().to_string();
    clean_help(raw_help, command, service)
}

fn clean_error(error: String) -> String {
    if let Some(idx) = error.find("\nFor more information try") {
        error
            .split_at(idx)
            .0
            .to_string()
            .replace("cmd ", "")
            .replace("SUBCOMMAND", "SERVICE")
            .trim()
            .to_string()
    } else {
        error.clone().replace("cmd ", "").trim().to_string()
    }
}

/// List of available services for rnotes command cli.
#[derive(StructOpt, Debug)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorNever])]
enum MainOpt {
    #[structopt(name = "auth")]
    /// Auth services.
    Auth(AuthOpt),
    #[structopt(name = "categories")]
    /// Categories services.
    Categories(CategoriesOpt),
    #[structopt(name = "notes")]
    /// Notes services.
    Notes(NotesOpt),
    /// Help services.
    #[structopt(name = "help")]
    Help(HelpOpt),
    /// Exit rnotes command cli.
    #[structopt(name = "exit")]
    Exit,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct HelpOpt {
    service: Option<String>,
    command: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
enum AuthOpt {
    /// Login to the server.
    #[structopt(name = "login")]
    Login(AuthLoginOpt),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct AuthLoginOpt {
    /// Email used to login.
    email: String,
    /// Password used to login.
    password: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
enum CategoriesOpt {
    /// Get all categories.
    #[structopt(name = "all")]
    All(CategoriesAllOpt),
    /// Get a category.
    #[structopt(name = "get")]
    Get(CategoriesGetOpt),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct CategoriesAllOpt {}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct CategoriesGetOpt {
    /// Id of the category.
    id: i32,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
enum NotesOpt {
    /// Get all notes.
    #[structopt(name = "all")]
    All(NotesAllOpt),
    /// Get a note.
    #[structopt(name = "get")]
    Get(NotesGetOpt),
    /// Create a note.
    #[structopt(name = "create")]
    Create(NotesCreateOpt),
    /// Get a note.
    #[structopt(name = "update")]
    Update(NotesUpdateOpt),
    /// Delete a note.
    #[structopt(name = "delete")]
    Delete(NotesDeleteOpt),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct NotesAllOpt {}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct NotesGetOpt {
    /// Id of the note.
    id: i32,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct NotesCreateOpt {
    /// Title of the note.
    title: String,
    /// Title of the note.
    data: String,
    /// Id of the category [Optional].
    category_id: Option<i32>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct NotesUpdateOpt {
    /// Id of the note.
    id: i32,
    /// Title of the note.
    title: String,
    /// Title of the note.
    data: String,
    /// Id of the category [Optional].
    category_id: Option<i32>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "_cmd", global_settings = &[AppSettings::ColorAuto])]
struct NotesDeleteOpt {
    /// Id of the note
    id: i32,
}

pub fn parse_command(_tokens: Vec<String>) -> Result<Command, Error> {
    if _tokens.len() == 0 {
        return Ok(Command::Nothing);
    }
    let service = _tokens[0].clone();
    let command = if _tokens.len() > 1 {
        Some(_tokens[1].clone())
    } else {
        None
    };

    let mut tokens = vec!["cmd".to_string()];
    tokens.extend(_tokens);

    match MainOpt::from_iter_safe(tokens.clone()) {
        Ok(MainOpt::Auth(auth)) => match auth {
            AuthOpt::Login(login) => Ok(Command::Auth(AuthCommand::Login(api::auth::LoginIn {
                email: login.email,
                password: login.password,
            }))),
        },
        Ok(MainOpt::Categories(categories)) => match categories {
            CategoriesOpt::All(_) => Ok(Command::Categories(CategoriesCommand::All)),
            CategoriesOpt::Get(get) => Ok(Command::Categories(CategoriesCommand::Get(get.id))),
        },
        Ok(MainOpt::Notes(notes)) => match notes {
            NotesOpt::All(_) => Ok(Command::Notes(NotesCommand::All)),
            NotesOpt::Get(get) => Ok(Command::Notes(NotesCommand::Get(get.id))),
            NotesOpt::Create(create) => {
                Ok(Command::Notes(NotesCommand::Create(api::note::NoteIn {
                    title: create.title,
                    data: create.data,
                    category_id: create.category_id,
                })))
            }
            NotesOpt::Update(update) => Ok(Command::Notes(NotesCommand::Update(
                update.id,
                api::note::NoteIn {
                    title: update.title,
                    data: update.data,
                    category_id: update.category_id,
                },
            ))),
            NotesOpt::Delete(delete) => Ok(Command::Notes(NotesCommand::Delete(delete.id))),
        },
        Ok(MainOpt::Help(HelpOpt { service, command })) => match (service, command) {
            (Some(service), None) => match &*service {
                "auth" => Ok(Command::Help(get_help(
                    &AuthOpt::clap(),
                    Some("auth".to_string()),
                    None,
                ))),
                "categories" => Ok(Command::Help(get_help(
                    &CategoriesOpt::clap(),
                    Some("categories".to_string()),
                    None,
                ))),
                "notes" => Ok(Command::Help(get_help(
                    &NotesOpt::clap(),
                    Some("notes".to_string()),
                    None,
                ))),
                _ => Err(Error::Parse(format!(
                    "error: service '{}' is not valid.",
                    service
                ))),
            },
            (Some(service), Some(command)) => match &*service {
                "auth" => match &*command {
                    "login" => Ok(Command::Help(get_help(
                        &AuthLoginOpt::clap(),
                        Some("auth".to_string()),
                        Some("login".to_string()),
                    ))),
                    _ => Err(Error::Parse(format!(
                        "error: command '{}' for service '{}' is not valid.",
                        command, service
                    ))),
                },
                "categories" => match &*command {
                    "all" => Ok(Command::Help(get_help(
                        &CategoriesAllOpt::clap(),
                        Some("categories".to_string()),
                        Some("all".to_string()),
                    ))),
                    "get" => Ok(Command::Help(get_help(
                        &CategoriesGetOpt::clap(),
                        Some("categories".to_string()),
                        Some("get".to_string()),
                    ))),
                    _ => Err(Error::Parse(format!(
                        "error: command '{}' for service '{}' is not valid.",
                        command, service
                    ))),
                },
                "notes" => match &*command {
                    "all" => Ok(Command::Help(get_help(
                        &NotesAllOpt::clap(),
                        Some("notes".to_string()),
                        Some("all".to_string()),
                    ))),
                    "get" => Ok(Command::Help(get_help(
                        &NotesGetOpt::clap(),
                        Some("notes".to_string()),
                        Some("get".to_string()),
                    ))),
                    "create" => Ok(Command::Help(get_help(
                        &NotesCreateOpt::clap(),
                        Some("notes".to_string()),
                        Some("create".to_string()),
                    ))),
                    "update" => Ok(Command::Help(get_help(
                        &NotesUpdateOpt::clap(),
                        Some("notes".to_string()),
                        Some("update".to_string()),
                    ))),
                    "delete" => Ok(Command::Help(get_help(
                        &NotesDeleteOpt::clap(),
                        Some("notes".to_string()),
                        Some("delete".to_string()),
                    ))),
                    _ => Err(Error::Parse(format!(
                        "error: command '{}' for service '{}' is not valid.",
                        command, service
                    ))),
                },
                _ => Err(Error::Parse(format!(
                    "error: service '{}' is not valid.",
                    service
                ))),
            },
            _ => Ok(Command::Help(get_help(&MainOpt::clap(), None, None))),
        },
        Ok(MainOpt::Exit) => Err(Error::Exit),
        Err(clap::Error {
            message,
            kind,
            info: _,
        }) => {
            if kind == clap::ErrorKind::UnknownArgument
                || kind == clap::ErrorKind::InvalidSubcommand
            {
                if command.is_some() {
                    Err(Error::Parse(format!(
                        "error: service '{}' and/or command '{}' are not valid.",
                        service,
                        command.unwrap()
                    )))
                } else {
                    Err(Error::Parse(format!(
                        "error: service '{}' is not valid.",
                        service
                    )))
                }
            } else if kind == clap::ErrorKind::MissingArgumentOrSubcommand {
                Err(Error::Parse(format!(
                    "error: missing command for service '{}' is not valid.",
                    service
                )))
            } else {
                Err(Error::Parse(clean_error(message)))
            }
        }
    }
}
