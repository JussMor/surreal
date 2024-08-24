use ntex::web::types::{State, Json};
use ntex::web::{ post, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::stores::tables_store::tables_store;
use crate::errors::app_errors::AppError;
use crate::AppState;

#[derive(Debug, Deserialize)]
struct BodyParams {
    table_name: String,
    table_type: String,
}

#[post("/post/db_documents")]
async fn create_db_table(
    state: State<AppState>,
    path: Json<BodyParams>,
) -> Result<HttpResponse, AppError> {
    let db = (*state.db).clone();

    let table_name = path.table_name.to_string();
    let table_type = path.table_type.to_string();


    let store = match tables_store::TablesStore::new(db).await {
        Ok(store) => store,
        Err(e) => return Err(AppError::InternalError(e.to_string()))
    };

    let tables = match store.create_table_storage(&table_type, &table_name).await {
        Ok(tables) => tables,
        Err(e) => return Err(AppError::InternalError(e.to_string()))
    };

    Ok(HttpResponse::Ok().body(json!({
        "code": 200,
        "message": "Table created",
        "tables": tables,
    })))
}