use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::Thing;
use std::collections::HashMap;

use super::blocks::BlockValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockComponentTemplate {
    pub id: Option<Thing>,
    pub name: String,
    pub default_data: HashMap<String, BlockValue>,
    pub default_display_config: HashMap<String, bool>,
}


impl BlockComponentTemplate {
    pub fn new(name: &str) -> Self {
        BlockComponentTemplate {
            id: None,
            name: String::from(name),
            default_data: HashMap::new(),
            default_display_config: HashMap::new(),
        }
    }

    
}