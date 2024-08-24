use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::models::templates::BlockComponentTemplate;
use crate::errors::app_errors::AppError;


struct BlockComponentTemplateStore {
    db: Surreal<Client>,
}

impl BlockComponentTemplateStore {
    async fn new(db: Surreal<Client>) -> Result<Self, AppError> {
        Ok(Self { db })
    }

    async fn create_template(
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
}