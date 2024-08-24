use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TablesFieldStore {
    pub name: String,
    pub table_type: String,
}
