use serde_json::json;
use derive_more::{Display, From};
use ntex::http::{self, StatusCode};
use ntex::web::{self, HttpResponse};
use surrealdb::Error as SurrealError;

#[derive(Debug, Display, From)]
pub enum AppError {
    #[display("internal error: {}", _0)]
    InternalError(String),

    #[display("bad request: {}", _0)]
    BadRequest(String),

    #[display("database error: {}", _0)]
    #[from]
    DatabaseError(SurrealError),

    #[display("not found: {}", _0)]
    NotFound(String),
}


impl std::error::Error for AppError {}

impl web::error::WebResponseError for AppError {
    fn error_response(&self,_: &web::HttpRequest ) -> web::HttpResponse {
        let (status, error_message) = match self {
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };

        HttpResponse::build(status)
            .body(json!({
                "error": error_message,
                "status": status.as_u16()
            }))
    }
}

