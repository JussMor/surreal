use surrealdb::engine::remote::ws::Client;
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


    pub async fn get_document(&self, table: &str, table_id: &str) -> Result<Option<BlockDocument>, AppError> {
        let result = self.db.select((table, table_id)).await?;

        match result {
            Some(doc) => Ok(Some(doc)),
            None => Ok(None),
        }
    }

    pub async fn merge_document(&self, table: &str, table_id: &str, partical_doc: BlockValue) -> Result<(), AppError> {
        self.db
            .update::<Option<BlockDocument>>((table, table_id))
            .merge(&partical_doc)
            .await?;
        Ok(())
    }

    pub async fn update_document(&self, table: &str, table_id: &str, doc: BlockDocument) -> Result<(), AppError> {
        self.db
            .update::<Option<BlockDocument>>((table, table_id))
            .content(&doc)
            .await?;
        Ok(())
    }
}
