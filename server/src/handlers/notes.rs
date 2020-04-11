use crate::handlers::jwt::JWTKey;
use crate::handlers::{status_error, ApiResponse, StatusError};
use crate::models::note::{NoteIn, NoteOut};

use rnotes_core::models::note::{NewNote, Note};
use rnotes_core::DBConn;

use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all<'r>(
    key: JWTKey,
    connection: DBConn,
) -> Result<ApiResponse<Vec<NoteOut>>, StatusError<'r>> {
    Note::find_by_user_id(&connection, key.id_user)
        .map(|all| ApiResponse::ok(all.iter().map(|note| NoteOut::from(note)).collect()))
        .map_err(|err| status_error(Status::NotFound, &format!("Cannot find notes: {}", err)))
}

#[get("/<id>")]
pub fn get<'r>(
    key: JWTKey,
    connection: DBConn,
    id: i32,
) -> Result<ApiResponse<NoteOut>, StatusError<'r>> {
    Note::find_by_id_and_user_id(&connection, id, key.id_user)
        .map(|note| ApiResponse::ok(NoteOut::from(&note)))
        .map_err(|err| status_error(Status::NotFound, &format!("Note is not correct: {}", err)))
}

#[post("/", format = "application/json", data = "<note_in>")]
pub fn post<'r>(
    key: JWTKey,
    connection: DBConn,
    note_in: Json<NoteIn>,
) -> Result<ApiResponse<NoteOut>, StatusError<'r>> {
    let mut new_note: NewNote = NoteIn::into(note_in.0);
    new_note.user_id = key.id_user;
    new_note
        .create(&connection)
        .map(|note| ApiResponse::new(NoteOut::from(&note), Status::Created))
        .map_err(|err| status_error(Status::BadRequest, &format!("Note is not correct: {}", err)))
}

#[put("/<id>", format = "application/json", data = "<note_in>")]
pub fn put<'r>(
    key: JWTKey,
    connection: DBConn,
    id: i32,
    note_in: Json<NoteIn>,
) -> Result<ApiResponse<NoteOut>, StatusError<'r>> {
    Note::find_by_id_and_user_id(&connection, id, key.id_user)
        .map_err(|err| status_error(Status::NotFound, &format!("Note is not correct: {}", err)))
        .and_then(|_| {
            let mut note: Note = NoteIn::into(note_in.0);
            note.id = id;
            note.user_id = key.id_user;
            Note::update(&connection, &note)
                .map(|note| ApiResponse::new(NoteOut::from(&note), Status::Created))
                .map_err(|err| {
                    status_error(Status::BadRequest, &format!("Note is not correct: {}", err))
                })
        })
}

#[delete("/<id>")]
pub fn delete<'r>(
    key: JWTKey,
    connection: DBConn,
    id: i32,
) -> Result<ApiResponse<&'r str>, StatusError<'r>> {
    Note::find_by_id_and_user_id(&connection, id, key.id_user)
        .and_then(|_| {
            Note::delete(&connection, id).map(|num| {
                ApiResponse::empty_new(if num > 0 {
                    Status::Ok
                } else {
                    Status::NoContent
                })
            })
        })
        .or(Ok(ApiResponse::empty_new(Status::NoContent)))
}
