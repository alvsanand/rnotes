pub mod auth;
pub mod category;
pub mod note;

#[derive(Debug, PartialEq, Serialize)]
pub struct Error {
    error: u16,
    detail: String,
}

impl Error {
    pub fn new(status: u16, error: String) -> Self {
        Error {
            error: status,
            detail: error,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Empty {}
