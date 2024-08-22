use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use std::collections::HashMap;

use super::blocks::BlockValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockComponentTemplate {
    id: Option<Thing>,
    name: String,
    default_data: HashMap<String, BlockValue>,
    default_display_config: HashMap<String, bool>,
}