use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::models::templates::BlockComponentTemplate;
use crate::errors::app_errors::AppError;


pub struct BlockComponentTemplateStore {
    db: Surreal<Client>,
}

impl BlockComponentTemplateStore {
    pub async fn new(db: Surreal<Client>) -> Result<Self, AppError> {
        Ok(Self { db })
    }

    pub async fn create_template(
        &self,
        template: &BlockComponentTemplate,
    ) -> Result<BlockComponentTemplate, AppError> {
        let doc: Vec<BlockComponentTemplate> = self.db.create("components").content(&template).await?;
        Ok(doc.first().cloned().ok_or(surrealdb::Error::Db(
            surrealdb::error::Db::RecordExists {
                thing: "The component created already exits".to_string(),
            },
        ))?)
    }

    pub async fn get_all_templates(&self) -> Result<Vec<BlockComponentTemplate>, AppError> {
        let templates: Result<Vec<BlockComponentTemplate>, surrealdb::Error>  = self.db.query("SELECT * FROM components").await?.take(0);
        
        match templates {
            Ok(templates) => {
                Ok(templates)
            },
            Err(e) => {
                log::error!("Failed to get all templates: {:?}", e);
                Err(AppError::DatabaseError(surrealdb::Error::Db(
                    surrealdb::error::Db::RecordExists  {
                        thing: "A Record with this name already exist".to_string(),
                    },
                )))
            }
        }
    }

}