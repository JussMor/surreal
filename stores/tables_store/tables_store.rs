use surrealdb::Surreal;
use surrealdb::Response;
use surrealdb::engine::remote::ws::Client;

use crate::models::blocks::BlockDocument;



pub struct TablesStore {
    db: Surreal<Client>,
}

impl  TablesStore {
    
    pub async fn new(db: Surreal<Client>) -> Result<Self, surrealdb::Error> {
        Ok(Self { db })
    }
    
    pub async fn get_tables_db(&self) -> Result<Vec<BlockDocument> , surrealdb::Error> {
        let database: Result<Vec<BlockDocument>, surrealdb::Error>= self.db.query("select * from jTEDDT").await?.take(0);
        match database {
            Ok(database) => {
                Ok(database)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
    
}