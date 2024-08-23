use surrealdb::Surreal;
use surrealdb::sql::Thing;
use surrealdb::engine::remote::ws::Client;

use crate::models::blocks::BlockDocument;

pub struct BlockDocumentStore {
    db: Surreal<Client>,
}

impl BlockDocumentStore {
    pub async fn new(db: Surreal<Client>) -> Result<Self, surrealdb::Error> {
        Ok(Self { db })
    }

    pub async fn create_document(&self, doc_name: &str, doc: BlockDocument) -> Result<BlockDocument, surrealdb::Error> {
        let doc: Vec<BlockDocument> = self.db.create(doc_name).content(&doc).await?;
        Ok(doc.first().cloned().ok_or(surrealdb::Error::Db(
            surrealdb::error::Db::RecordExists {
                thing: "The document create document already exits".to_string(),
            },
        ))?)
    }

    pub async fn get_document(&self, id: &Thing) -> Result<Option<BlockDocument>, surrealdb::Error> {
        let result = self.db.select(("documents", id.id.to_raw())).await?;
        Ok(result)
    }

    pub async fn update_document(&self, id: &Thing, doc: BlockDocument) -> Result<(), surrealdb::Error> {
        self.db
            .update::<Option<BlockDocument>>(("documents", id.id.to_raw()))
            .content(&doc)
            .await?;
        Ok(())
    }
}

