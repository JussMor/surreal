use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::errors::app_errors::AppError;
use crate::models::tables::TablesFieldStore;

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

    pub async fn get_table_by_name(&self, table_name: &str) -> Result<Vec<TablesFieldStore>, AppError> {

        let table:Result<Vec<TablesFieldStore>, surrealdb::Error> = self.db.query("SELECT id, name, table_type FROM tables_storage WHERE name = $name")
            .bind(("name", table_name))
        .await.unwrap().take(0);
        
        match table {
            Ok(table) => {
                Ok(table)
            },
            Err(e) => {
                log::error!("Failed to get table by name: {:?}", e);
                Err(AppError::DatabaseError(surrealdb::Error::Db(
                    surrealdb::error::Db::RecordExists  {
                        thing: "A Record with this name already exist".to_string(),
                    },
                )))
            }
        }
    }

    pub async fn create_table_storage(
        &self,
        name: &str,
    ) -> Result<Vec<TablesFieldStore>, AppError> {
        let new_table = TablesFieldStore {
            id: None,
            name: name.to_string(),
        };
        match self.db.create("tables_storage").content(new_table).await {
            Ok(database) => Ok(database),
            Err(e) => {
                log::error!("Failed to create table storage: {:?}", e);
                Err(AppError::DatabaseError(surrealdb::Error::Db(
                    surrealdb::error::Db::RecordExists {
                        thing: e.to_string(),
                    },
                )))
            }
        }
    }


    pub async fn delete_table_storage(
        &self,
        table_name: &str,
        table_id: &str,
    ) -> Result<(), AppError> {
        let delete: Result<Option<TablesFieldStore>, surrealdb::Error> = self.db.delete((table_name, table_id)).await;
        
        match delete {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Failed to delete table storage: {:?}", e);
                Err(AppError::DatabaseError(surrealdb::Error::Db(
                    surrealdb::error::Db::RecordExists {
                        thing: e.to_string(),
                    },
                )))
            }
        }
    }



}
