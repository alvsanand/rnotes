use rocket::fairing::AdHoc;
use rocket::http::hyper::header::ContentLength;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use rocket_contrib::json::*;

use serde::Serialize;
use std::io::Cursor;

use rnotes_core::models::api::Error;

pub mod auth;
pub mod categories;
pub mod jwt;
pub mod notes;

pub fn catch_not_json() -> AdHoc {
    AdHoc::on_response("catch_errors", |_, res| {
        if res.status().code >= 400
            && ContentType::parse_flexible(res.headers().get_one("Content-Type").unwrap())
                != Some(ContentType::JSON)
        {
            let string_error: Error =
                Error::new(res.status().code, res.status().reason.to_string());

            let json = serde_json::to_string(&string_error).unwrap();
            let content_length = json.len() as u64;

            res.set_header(ContentLength(content_length));
            res.set_sized_body(Cursor::new(json));
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

pub type StatusError<'a> = Response<'a>;

pub fn status_error<'r>(status: Status, error: String) -> StatusError<'r> {
    let string_error: Error = Error::new(status.code, error);

    json_response(&string_error, status)
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub json: Option<Json<T>>,
    pub status: Status,
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
