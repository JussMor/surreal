use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TablesFieldStore {
    pub id: Option<Thing>,
    pub name: String,
    pub table_type: String,
}
