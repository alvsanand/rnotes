use crate::cmd::*;
use crate::http_client::HttpClient;
use rnotes_core::models::api::auth::*;
use rnotes_core::models::api::category::CategoryOut;
use rnotes_core::models::api::note::*;
use rnotes_core::models::api::Empty;
use rnotes_core::utils::HexSlice;
use sha2::{Digest, Sha256};

pub struct Runner {
    server_url: String,
    http_client: HttpClient,
    jwt_token: Option<String>,
}

impl Runner {
    pub fn new(server_url: String, http_client: HttpClient) -> Self {
        Runner {
            server_url: server_url,
            http_client: http_client,
            jwt_token: None,
        }
    }

    pub async fn run(&mut self, cmd: Command) -> String {
        match cmd {
            Command::Help(message) => format!("{}", message),
            Command::Auth(AuthCommand::Login(mut login_in)) => {
                let url = format!("{server}/auth/login", server = self.server_url);

                let mut hasher = Sha256::new();
                hasher.input(login_in.password.into_bytes());
                let hashed_password = HexSlice::new(&hasher.result()).to_string();

                login_in.password = hashed_password;

                match self
                    .http_client
                    .post::<LoginIn, LoginOut>(url, &login_in, None)
                    .await
                {
                    Ok(login_out) => {
                        self.jwt_token = Some(login_out.jwt_token);
                        format!("Login successful to rnotes server.")
                    }
                    Err(err) => format!("Failed 'auth login'. {err}", err = err),
                }
            }
            Command::Categories(CategoriesCommand::All) => {
                let url = format!("{server}/categories/", server = self.server_url);

                match self
                    .http_client
                    .get::<Vec<CategoryOut>>(url, self.jwt_token.clone())
                    .await
                {
                    Ok(response) => format!("{:?}", response),
                    Err(err) => format!("Failed 'categories all'. {err}", err = err),
                }
            }
            Command::Categories(CategoriesCommand::Get(id)) => {
                let url = format!(
                    "{server}/categories/{id}",
                    server = self.server_url,
                    id = id
                );

                match self
                    .http_client
                    .get::<CategoryOut>(url, self.jwt_token.clone())
                    .await
                {
                    Ok(response) => format!("{:?}", response),
                    Err(err) => format!("Failed 'categories get {id}'. {err}", id = id, err = err),
                }
            }
            Command::Notes(NotesCommand::All) => {
                let url = format!("{server}/notes/", server = self.server_url);

                match self
                    .http_client
                    .get::<Vec<NoteOut>>(url, self.jwt_token.clone())
                    .await
                {
                    Ok(response) => format!("{:?}", response),
                    Err(err) => format!("Failed 'notes all'. {}", err),
                }
            }
            Command::Notes(NotesCommand::Get(id)) => {
                let url = format!("{server}/notes/{id}", server = self.server_url, id = id);

                match self
                    .http_client
                    .get::<NoteOut>(url, self.jwt_token.clone())
                    .await
                {
                    Ok(response) => format!("{:?}", response),
                    Err(err) => format!("Failed 'notes get {id}'. {err}", id = id, err = err),
                }
            }
            Command::Notes(NotesCommand::Create(note)) => {
                let url = format!("{server}/notes", server = self.server_url);

                match self
                    .http_client
                    .post::<NoteIn, NoteOut>(url, &note, self.jwt_token.clone())
                    .await
                {
                    Ok(response) => format!("{:?}", response),
                    Err(err) => format!("Failed 'notes create ...'. {err}", err = err),
                }
            }
            Command::Notes(NotesCommand::Update(id, note)) => {
                let url = format!("{server}/notes/{id}", server = self.server_url, id = id);

                match self
                    .http_client
                    .put::<NoteIn, NoteOut>(url, &note, self.jwt_token.clone())
                    .await
                {
                    Ok(response) => format!("{:?}", response),
                    Err(err) => {
                        format!("Failed 'notes update {id} ...'. {err}", id = id, err = err)
                    }
                }
            }
            Command::Notes(NotesCommand::Delete(id)) => {
                let url = format!("{server}/notes/{id}", server = self.server_url, id = id);

                match self
                    .http_client
                    .delete::<Empty>(url, self.jwt_token.clone())
                    .await
                {
                    Ok(response) => format!("{:?}", response),
                    Err(err) => format!("Failed 'notes delete {id}'. {err}", id = id, err = err),
                }
            }
            other => format!("Received {:?}", other),
        }
    }
}
