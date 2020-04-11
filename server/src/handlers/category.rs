use crate::handlers::jwt::JWTKey;
use crate::handlers::{status_error, ApiResponse, StatusError};
use crate::models::category::CategoryOut;

use rnotes_core::models::category::Category;
use rnotes_core::DBConn;

use rocket::http::Status;

#[get("/")]
pub fn all<'r>(
    _key: JWTKey,
    connection: DBConn,
) -> Result<ApiResponse<Vec<CategoryOut>>, StatusError<'r>> {
    Category::find_all(&connection)
        .map(|all| {
            ApiResponse::ok(
                all.iter()
                    .map(|category| CategoryOut::from(category))
                    .collect(),
            )
        })
        .map_err(|err| {
            status_error(
                Status::NotFound,
                &format!("Cannot find categories: {}", err),
            )
        })
}

#[get("/<id>")]
pub fn get<'r>(
    _key: JWTKey,
    connection: DBConn,
    id: i32,
) -> Result<ApiResponse<CategoryOut>, StatusError<'r>> {
    Category::find_by_id(&connection, id)
        .map(|category| ApiResponse::ok(CategoryOut::from(&category)))
        .map_err(|err| {
            status_error(
                Status::NotFound,
                &format!("Category is not correct: {}", err),
            )
        })
}
