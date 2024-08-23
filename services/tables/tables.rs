use std::collections::HashMap;

use crate::models::blocks::BlockDocument;
use crate::stores::block_documents_store::document_store;
use crate::AppState;
use log::{error, info};
use ntex::web::types::{Path, State};
use ntex::web::{get, Error, HttpResponse};
use serde::Deserialize;
use serde_json::{json};
use surrealdb::sql::Value;

use crate::stores::tables_store::tables_store;


#[derive(Debug, Deserialize)]
struct Response {
    databases: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct PathParams {
    db_name: String,
    doc_name: String,
}

#[get("/get/db_documents")]
async fn get_db_tables(state: State<AppState>) -> Result<HttpResponse, Error> {
    let db = (*state.db).clone();
    
    let store = tables_store::TablesStore::new(db).await;
    
    match store {
        Ok(store) => {
            let db = store.get_tables_db().await;
            match db {
                Ok(db) => {
                    println!("soy db {}", db[0].name);
                    Ok(HttpResponse::Ok().body(json!({
                        "code": 200,
                        "message": db,
                    })))
                },
                Err(e) => {
                    Ok(HttpResponse::InternalServerError().body(json!({
                        "code": 500,
                        "message": "Error creating block document",
                        "error": e.to_string()})))
                }
            }
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError().body(json!({
                "code": 500,
                "message": "Error setting the document store",
                "error": e.to_string()})))
        }
    }
}


