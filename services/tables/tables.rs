use ntex::web::types::{Json, State};
use ntex::web::{delete, post, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::errors::app_errors::AppError;
use crate::stores::tables_store::tables_store;
use crate::AppState;

#[derive(Debug, Deserialize)]
struct BodyParams {
    table_name: String
}

#[post("/post/db_documents")]
async fn create_db_table(
    state: State<AppState>,
    path: Json<BodyParams>,
) -> Result<HttpResponse, AppError> {
    let db = (*state.db).clone();

    let table_name = path.table_name.to_string();

    let store = match tables_store::TablesStore::new(db).await {
        Ok(store) => store,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    let tables = match store.create_table_storage(&table_name).await {
        Ok(tables) => tables,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    Ok(HttpResponse::Ok().body(json!({
        "code": 200,
        "message": "Table created",
        "tables": tables,
    })))
}


#[derive(Debug, Deserialize)]
struct DeleteBodyParams {
    table_name: String,
    table_id: String,
}

#[delete("/delete/db_documents")]
async fn delete_db_table(
    state: State<AppState>,
    path: Json<DeleteBodyParams>,
) -> Result<HttpResponse, AppError> {
    let db = (*state.db).clone();

    let table_name = path.table_name.to_string();
    let table_id = path.table_id.to_string();

    let store = match tables_store::TablesStore::new(db).await {
        Ok(store) => store,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    match store.delete_table_storage(&table_name, &table_id).await {
        Ok(tables) => tables,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    Ok(HttpResponse::Ok().body(json!({
        "code": 200,
        "message": "Table storage deleted",
    })))
}
