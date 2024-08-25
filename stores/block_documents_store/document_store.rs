use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::errors::app_errors::AppError;
use crate::models::blocks::{BlockDocument, BlockValue};

pub struct BlockDocumentStore {
    db: Surreal<Client>,
}

impl BlockDocumentStore {
    pub async fn new(db: Surreal<Client>) -> Result<Self, AppError> {
        Ok(Self { db })
    }

    pub async fn create_document(
        &self,
        doc_name: &str,
        doc: BlockDocument,
    ) -> Result<BlockDocument, AppError> {
        let doc: Vec<BlockDocument> = self.db.create(doc_name).content(&doc).await?;

        let document = match doc.first() {
            Some(doc) => Ok(doc.clone()),
            None => Err(AppError::DatabaseError(surrealdb::Error::Db(
                surrealdb::error::Db::RecordExists {
                    thing: "The document create document already exits".to_string(),
                },
            ))),
        };

        document
    }

    #[allow(dead_code)]
    pub async fn get_document(&self, id: &Thing) -> Result<Option<BlockDocument>, AppError> {
        let result = self.db.select(("documents", id.id.to_raw())).await?;
        Ok(result)
    }

    pub async fn update_document(&self, table: &str, table_id: &str, partical_doc: BlockValue) -> Result<(), AppError> {
        self.db
            .update::<Option<BlockDocument>>((table, table_id))
            .merge(&partical_doc)
            .await?;
        Ok(())
    }
}
