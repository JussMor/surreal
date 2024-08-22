use ntex::web::{self, get, HttpResponse,Error};
use ntex::web::types::{Path, Query, State};


#[get("/create")]
async fn create_block() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Create block"))
}