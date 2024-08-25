use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use surrealdb::sql::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockValue {
    Null,
    None,
    Vec(Vec<BlockValue>),
    Boolean(bool),
    String(String),
    Number(f64),
    Thing(Thing),
    Object(HashMap<String, BlockValue>),
}

pub type BlockData = HashMap<String, BlockValue>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub data: HashMap<String, BlockValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub block_id: String,
    pub timestamp: DateTime<Utc>,
    pub old_data: HashMap<String, BlockValue>,
    pub new_data: HashMap<String, BlockValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockDocumentContent {
    pub time: i64,
    pub blocks: Vec<Block>,
    pub version: String,
}

// Add an attribute independent from the id field
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub name: String,
    pub content: BlockDocumentContent,
    pub changes: Vec<Change>,
}

impl BlockDocument {
    pub fn new(name: &str) -> Self {
        BlockDocument {
            id: None,
            name: String::from(name),
            content: BlockDocumentContent {
                time: Utc::now().timestamp_millis(),
                blocks: Vec::new(),
                version: "2.22.2".to_string(),
            },
            changes: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block_type: &str, data: HashMap<String, BlockValue>) -> String {
        let id = Uuid::new_v4().to_string();
        let block = Block {
            id: id.clone(),
            block_type: block_type.to_string(),
            data,
        };
        self.content.blocks.push(block);
        id
    }

    // testear this to check if it working with new impl
    pub fn update_block(
        &mut self,
        block_id: &str,
        new_data: HashMap<String, BlockValue>,
    ) -> Result<(), String> {
        if let Some(block) = self.content.blocks.iter_mut().find(|b| b.id == block_id) {
            let old_data = std::mem::replace(&mut block.data, new_data.clone());
            let change = Change {
                block_id: block_id.to_string(),
                timestamp: Utc::now(),
                old_data,
                new_data,
            };
            self.changes.push(change);
            Ok(())
        } else {
            Err("Block not found".to_string())
        }
    }
}


