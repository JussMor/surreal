use ntex::web::WebResponseError;
use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Response;
use surrealdb::Surreal;

use crate::errors::app_errors::AppError;
use crate::models::blocks::BlockDocument;
use crate::models::tables::TablesFieldStore;
use ntex::web::{self, post, Error, HttpResponse};

pub struct TablesStore {
    db: Surreal<Client>,
}

impl TablesStore {
    pub async fn new(db: Surreal<Client>) -> Result<Self, AppError> {
        Ok(Self { db })
    }

    // pub async fn get_tables_db(&self) -> Result<Vec<BlockDocument> , surrealdb::Error> {
    //     let database: Result<Vec<BlockDocument>, surrealdb::Error>= self.db.query("select * from jTEDDT").await?.take(0);
    //     match database {
    //         Ok(database) => {
    //             Ok(database)
    //         },
    //         Err(e) => {
    //             Err(e)
    //         }
    //     }
    // }

    pub async fn create_table_storage(
        &self,
        table_type: &str,
        name: &str,
    ) -> Result<Vec<TablesFieldStore>, AppError> {
        let new_table = TablesFieldStore {
            table_type: table_type.to_string(),
            name: name.to_string(),
        };
        match self.db.create("tables_storage").content(new_table).await {
            Ok(database) => Ok(database),
            Err(e) => {
                log::error!("Failed to create table storage: {:?}", e);
                Err(AppError::DatabaseError(surrealdb::Error::Db(
                    surrealdb::error::Db::RecordExists {
                        thing: "The table created already exits".to_string(),
                    },
                )))
            }
        }
    }
}
