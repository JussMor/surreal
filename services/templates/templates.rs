use ntex::web::types::{Json, State};
use ntex::web::{post,get, HttpResponse};
use serde_json::json;

use crate::models::templates::BlockComponentTemplate;
use crate::stores::block_templates_store::template_store;
use crate::errors::app_errors::AppError;
use crate::AppState;


#[post("/post/templates")]
async fn create_template(
    state: State<AppState>,
    data: Json<BlockComponentTemplate>,
) -> Result<HttpResponse, AppError> {
    let db = (*state.db).clone();


    let store = match template_store::BlockComponentTemplateStore::new(db).await {
        Ok(store) => store,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    let templates = match store.create_template(&data).await {
        Ok(templates) => templates,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };
    
    Ok(HttpResponse::Ok().body(json!({
        "code": 200,
        "message": "Template created",
        "templates": templates,
    })))
}


#[get("/get/templates")]
async fn get_all_templates(
    state: State<AppState>,
) -> Result<HttpResponse, AppError> {
    let db = (*state.db).clone();

    let store = match template_store::BlockComponentTemplateStore::new(db).await {
        Ok(store) => store,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    let templates = match store.get_all_templates().await {
        Ok(templates) => templates,
        Err(e) => return Err(AppError::InternalError(e.to_string())),
    };

    Ok(HttpResponse::Ok().body(json!({
        "code": 200,
        "message": "Templates fetched",
        "templates": templates,
    })))
}