use log::{error, info};
use ntex::web::types::{Json, State};
use ntex::web::{post, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use surrealdb::sql::{Id, Thing};

use crate::errors::app_errors::AppError;
use crate::models::blocks::{BlockData, BlockDocument, BlockValue};
use crate::stores::block_documents_store::document_store;
use crate::stores::tables_store::tables_store;
use crate::AppState;

#[derive(Debug, Deserialize)]
struct BodyParams {
    db_name: String,
    doc_name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateBlocksWithCompnentParams {
    table: String,
    table_id: String,
    component_id: String,
}

#[post("/post/block_document")]
async fn create_block(
    state: State<AppState>,
    data: Json<BodyParams>,
) -> Result<HttpResponse, AppError> {
    let db_name = data.db_name.to_string();
    let doc_name = data.doc_name.to_string();

    let db = (*state.db).clone();

    let store = match document_store::BlockDocumentStore::new(db.clone()).await {
        Ok(store) => store,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    let tables = match tables_store::TablesStore::new(db).await {
        Ok(tables) => tables,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    let doc_up: BlockDocument = BlockDocument::new(&doc_name);

    let document = match store.create_document(&db_name, doc_up).await {
        Ok(doc) => doc,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    // Take the ID of the document and create a block document with the table_storage_id
    match document.id.clone() {
        Some(id) => {
            info!("Block Document created with ID: {:?}", id.tb);
            let check_table = tables.get_table_by_name(&id.tb.to_string()).await?;
            let update_block = BlockValue::Object(BlockData::from([(
                "table_storage_id".to_string(),
                BlockValue::Thing(Thing {
                    tb: check_table[0].id.clone().unwrap().tb,
                    id: Id::from(check_table[0].id.clone().unwrap().id),
                }),
            )]));

            store
                .merge_document(
                    &db_name,
                    &document.id.clone().unwrap().id.to_string(),
                    update_block,
                )
                .await?;
        }
        None => {
            error!("Block Document, but ID is not available");
        }
    };

    Ok(HttpResponse::Ok().body(json!({
        "document": document,
    })))
}

#[post("/post/update_block_document_with_component")]
async fn update_block_with_component(
    state: State<AppState>,
    data: Json<UpdateBlocksWithCompnentParams>,
) -> Result<HttpResponse, AppError> {
    let db = (*state.db).clone();

    let data = UpdateBlocksWithCompnentParams {
        table: data.table.to_string(),
        table_id: data.table_id.to_string(),
        component_id: data.component_id.to_string(),
    };

    let store = match document_store::BlockDocumentStore::new(db).await {
        Ok(store) => store,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    let mut get_document = match store.get_document(&data.table, &data.table_id).await {
        Ok(doc) => match doc {
            Some(doc) => doc,
            None => return Err(AppError::InternalError("Document not found".to_string())),
        },
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    get_document.add_block(
        "custom_template",
        BlockData::from([(
            "component_template_id".to_string(),
            BlockValue::Thing(Thing {
                tb: "components".to_string(),
                id: Id::from(&data.component_id),
            }),
        )]),
    );

    match store
        .update_document(&data.table, &data.table_id, get_document)
        .await
    {
        Ok(doc) => doc,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    Ok(HttpResponse::Ok().body(json!({
        "document": "Document Updated 2 Successfully",
    })))
}
