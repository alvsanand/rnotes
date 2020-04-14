use rocket::fairing::AdHoc;
use rocket::http::hyper::header::ContentLength;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use rocket_contrib::json::*;

use serde::Serialize;
use std::io::Cursor;

pub mod auth;
pub mod category;
pub mod jwt;
pub mod notes;

pub fn catch_not_json() -> AdHoc {
    AdHoc::on_response("catch_errors", |_, res| {
        if res.status().code >= 400
            && ContentType::parse_flexible(res.headers().get_one("Content-Type").unwrap())
                != Some(ContentType::JSON)
        {
            let string_error: StringError = StringError::new(res.status(), res.status().reason);

            let json = serde_json::to_string(&string_error).unwrap();
            let content_length = json.len() as u64;

            res.set_header(ContentLength(content_length));
            res.set_sized_body(Cursor::new(json));
            res.set_status(res.status());
        }
    })
}

pub fn json_response<'a, T: Serialize>(t: &T, status: Status) -> Response<'a> {
    let json = serde_json::to_string(t).unwrap();
    let content_length = json.len() as u64;

    Response::build()
        .header(ContentType::JSON)
        .header(ContentLength(content_length))
        .sized_body(Cursor::new(json))
        .status(status)
        .finalize()
}

#[derive(Serialize)]
struct StringError<'a> {
    error: u16,
    detail: &'a str,
}

impl<'a> StringError<'a> {
    pub fn new(status: Status, error: &'a str) -> Self {
        StringError {
            error: status.code,
            detail: error,
        }
    }
}

pub type StatusError<'a> = Response<'a>;

pub fn status_error<'a, 'r>(status: Status, error: &'a str) -> StatusError<'r> {
    let string_error: StringError<'a> = StringError::new(status, error);

    json_response(&string_error, status)
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    json: Option<Json<T>>,
    status: Status,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn empty_ok() -> ApiResponse<T> {
        ApiResponse::<T> {
            json: None,
            status: Status::Ok,
        }
    }

    pub fn empty_new(status: Status) -> ApiResponse<T> {
        ApiResponse::<T> {
            json: None,
            status: status,
        }
    }
    pub fn ok(value: T) -> ApiResponse<T> {
        ApiResponse::<T> {
            json: Some(Json(value)),
            status: Status::Ok,
        }
    }

    pub fn new(value: T, status: Status) -> ApiResponse<T> {
        ApiResponse::<T> {
            json: Some(Json(value)),
            status: status,
        }
    }
}

impl<'r, T: Serialize> Responder<'r> for ApiResponse<T> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        if let Some(_json) = self.json {
            Response::build_from(_json.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        } else {
            Response::build().status(self.status).ok()
        }
    }
}