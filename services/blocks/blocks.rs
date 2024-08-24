use ntex::web::{post, HttpResponse,Error};
use ntex::web::types::{Path, State};
use log::{info, error};
use serde_json::json;
use serde::Deserialize;
use crate::models::blocks::BlockDocument;
use crate::AppState;
use crate::stores::block_documents_store::document_store;


#[derive(Debug, Deserialize)]
struct PathParams {
    db_name: String,
    doc_name:String,
}


#[post("/create/block_document/{db_name}/{doc_name}")]
async fn create_block(state: State<AppState>,path: Path<PathParams>) -> Result<HttpResponse, Error> {
    
    let id = path.db_name.to_string();
    let name = path.doc_name.to_string();
    
    
    let db = (*state.db).clone();
    let store = document_store::BlockDocumentStore::new(db).await;
    let  doc_up: BlockDocument = BlockDocument::new(&name);
    
    match store {
        Ok(store) => {
            let doc = store.create_document(&id, doc_up).await;
            match doc {
                Ok(doc) => {
                    match doc.id {
                        Some(id) => info!("Block Document created with ID: {}", id),
                        None => error!("Block Document, but ID is not available"),
                    }
                    Ok(HttpResponse::Ok().body(json!({
                        "code": 200,
                        "message": "Block Document created",
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