use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::models::templates::BlockComponentTemplate;


struct BlockComponentTemplateStore {
    db: Surreal<Client>,
}

impl BlockComponentTemplateStore {
    async fn new(db: Surreal<Client>) -> Result<Self, surrealdb::Error> {
        Ok(Self { db })
    }

    async fn create_template(
        &self,
        template: &BlockComponentTemplate,
    ) -> Result<BlockComponentTemplate, surrealdb::Error> {
        let doc: Vec<BlockComponentTemplate> = self.db.create("components").content(&template).await?;
        Ok(doc.first().cloned().ok_or(surrealdb::Error::Db(
            surrealdb::error::Db::RecordExists {
                thing: "The component created already exits".to_string(),
            },
        ))?)
    }
}