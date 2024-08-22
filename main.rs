use std::any::Any;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use ntex_cors::Cors;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{thing, Id, Uuid};
use surrealdb::sql::{Thing, Value};
use surrealdb::Surreal;
use ntex::web::{App, HttpServer, middleware::Logger};
use env_logger::Env;
use std::env;
use std::sync::Arc;

mod services;
mod db;
mod models;
mod stores;

#[derive(Clone)]
struct AppState {
    db: Arc<Surreal<Client>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ComponentTemplate {
    id: Option<Thing>,
    name: String,
    default_data: HashMap<String, BlockValue>,
    default_display_config: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum BlockValue {
    Null,
    None,
    Vec(Vec<BlockValue>),
    Boolean(bool),
    String(String),
    Number(f64),
    Thing(Thing),
    Object(HashMap<String, BlockValue>),
}

type BlockData = HashMap<String, BlockValue>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    id: String,
    #[serde(rename = "type")]
    block_type: String,
    data: HashMap<String, BlockValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Change {
    block_id: String,
    timestamp: DateTime<Utc>,
    old_data: HashMap<String, BlockValue>,
    new_data: HashMap<String, BlockValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DocumentContent {
    time: i64,
    blocks: Vec<Block>,
    version: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Document {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Thing>,
    content: DocumentContent,
    changes: Vec<Change>,
}
struct DocumentStore {
    db: Surreal<Client>,
}

impl DocumentStore {
    async fn new(db: Surreal<Client>) -> Result<Self, surrealdb::Error> {
        Ok(Self { db })
    }

    async fn create_document(&self, doc: Document) -> Result<Document, surrealdb::Error> {
        let doc: Vec<Document> = self.db.create("documents").content(&doc).await?;
        Ok(doc.first().cloned().ok_or(surrealdb::Error::Db(
            surrealdb::error::Db::RecordExists {
                thing: "The document create document already exits".to_string(),
            },
        ))?)
    }

    async fn get_document(&self, id: &Thing) -> Result<Option<Document>, surrealdb::Error> {
        let result = self.db.select(("documents", id.id.to_raw())).await?;
        Ok(result)
    }

    async fn update_document(&self, id: &Thing, doc: Document) -> Result<(), surrealdb::Error> {
        self.db
            .update::<Option<Document>>(("documents", id.id.to_raw()))
            .content(&doc)
            .await?;
        Ok(())
    }
}

impl Document {
    fn new() -> Self {
        Document {
            id: None,
            content: DocumentContent {
                time: Utc::now().timestamp_millis(),
                blocks: Vec::new(),
                version: "2.22.2".to_string(),
            },
            changes: Vec::new(),
        }
    }

    fn add_block(&mut self, block_type: &str, data: HashMap<String, BlockValue>) -> String {
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
    fn update_block(
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

struct ComponentTemplateStore {
    db: Surreal<Client>,
}

impl ComponentTemplateStore {
    async fn new(db: Surreal<Client>) -> Result<Self, surrealdb::Error> {
        Ok(Self { db })
    }

    async fn create_template(
        &self,
        template: &ComponentTemplate,
    ) -> Result<ComponentTemplate, surrealdb::Error> {
        let doc: Vec<ComponentTemplate> = self.db.create("components").content(&template).await?;
        Ok(doc.first().cloned().ok_or(surrealdb::Error::Db(
            surrealdb::error::Db::RecordExists {
                thing: "The component created already exits".to_string(),
            },
        ))?)
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // Connect to the server
    let connection = match db::connection::function_connect_to_surreal().await {
        Ok(db) => {
            // Proceed with your logic using the `db` connection
            println!("Successfully connected to the database.");
            db
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
    
    let db = Arc::new(connection);
    
    

    // let doc = Document {
    //     id: None,
    //     content: DocumentContent {
    //         time: Utc::now().timestamp_millis(),
    //         blocks: vec![
    //             Block {
    //                 id: "1".to_string(),
    //                 block_type: "text".to_string(),
    //                 data: BlockData::from([(
    //                     "text".to_string(),
    //                     BlockValue::String("hola".to_string()),
    //                 )]),
    //             },
    //             Block {
    //                 id: "2".to_string(),
    //                 block_type: "thing".to_string(),
    //                 data: BlockData::from([(
    //                     "template_id".to_string(),
    //                     BlockValue::Thing(Thing {
    //                         tb: "hola".to_string(),
    //                         id: Id::from("1".to_string()),
    //                     }),
    //                 )]),
    //             },
    //             Block {
    //                 id: "2".to_string(),
    //                 block_type: "thing".to_string(),
    //                 data: BlockData::from([(
    //                     "firs_top".to_string(),
    //                     BlockValue::Object(BlockData::from([(
    //                         "nested_field".to_string(),
    //                         BlockValue::Thing(Thing {
    //                             tb: "hola".to_string(),
    //                             id: Id::from("1".to_string()),
    //                         }),
    //                     )])),
    //                 )]),
    //             },
    //         ],
    //         version: "1.0".to_string(),
    //     },
    //     changes: Vec::new(),
    // };

    // let doc2: Vec<Document> = db.create("documents").content(&doc).await?;

    // println!("{:?}", doc2[0].id.id.to_string());

    // let update: Option<Document> = db
    //     .update(("documents", "19"))
    //     .content({
    //         let mut doc = doc.clone();
    //         doc.content.blocks[0].data.insert("template_id".to_string(), BlockValue::String("what ".to_string()));
    //         doc
    //     })
    //     .await?;

    //Upbase Code
    // let store: DocumentStore = DocumentStore::new(db.clone()).await?;

    // let mut doc_up: Document = Document::new();

    // doc_up.add_block(
    //     "header",
    //     BlockData::from([("text".to_string(), BlockValue::String("hola".to_string()))]),
    // );

    // // Save the document to the database
    // let create_doc: Document = store.create_document(doc_up).await?;

    // let doc_id = create_doc.id.unwrap();

    // let mut retrieved_doc = store.get_document(&doc_id).await?.unwrap();


    // let template_store = ComponentTemplateStore::new(db).await?;

    // let author_bio_template = ComponentTemplate {
    //     id: Some(Thing::from(("component_templates", "ct-001"))),
    //     name: "Author Bio".to_string(),
    //     default_data: BlockData::from([
    //         (
    //             "name".to_string(),
    //             BlockValue::String("Default Author".to_string()),
    //         ),
    //         (
    //             "bio".to_string(),
    //             BlockValue::String("Default Author Bio".to_string()),
    //         ),
    //         (
    //             "image_url".to_string(),
    //             BlockValue::String("https://example.com/default.jpg".to_string()),
    //         ),
    //         (
    //             "social_links".to_string(),
    //             BlockValue::Vec(vec![
    //                 BlockValue::Object(BlockData::from([(
    //                     "nested_field".to_string(),
    //                     BlockValue::Thing(Thing {
    //                         tb: "documents".to_string(),
    //                         id: Id::from("euqgnw19107bsgeprzoe".to_string()),
    //                     }),
    //                 )])),
    //                 BlockValue::Number(2.0),
    //             ]),
    //         ),
    //     ]),
    //     default_display_config: HashMap::from([
    //         ("name".to_string(), true),
    //         ("bio".to_string(), true),
    //         ("image_url".to_string(), true),
    //         ("social_links".to_string(), true),
    //     ]),
    // };

    // let template_id = template_store.create_template(&author_bio_template).await?;


    // retrieved_doc.add_block(
    //         "paragraph",
    //         BlockData::from([("text".to_string(), BlockValue::String("updatesjdfals;djfa".to_string()))]),
    // );

    let state = AppState {
        db: db,
    };
    
     // store.update_document(&doc_id, retrieved_doc).await?;

     
     HttpServer::new(move || {
         App::new()
         .state(state.clone())
         .wrap(Cors::new().allowed_origin("*").finish())
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
             .configure(services::blocks::router::config)
     })
     .bind("127.0.0.1:3030")?
     .run()
     .await
}
