// use futures_util::{ SinkExt, StreamExt};
// use warp::Filter;
// use warp::ws::{Message, WebSocket};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ComponentTemplate {
    id: Thing,
    name: String,
    default_data: HashMap<String, Value>,
    default_display_config: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    id: String,
    #[serde(rename = "type")]
    block_type: String,
    data: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Change {
    block_id: String,
    timestamp: DateTime<Utc>,
    old_data: HashMap<String, Value>,
    new_data: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentContent {
    time: i64,
    blocks: Vec<Block>,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    id: Option<Thing>,
    content: DocumentContent,
    changes: Vec<Change>,
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

    fn add_block(&mut self, block_type: &str, data: HashMap<String, Value>) -> String {
        let id = Uuid::new_v4().to_string();
        let block = Block {
            id: id.clone(),
            block_type: block_type.to_string(),
            data,
        };
        self.content.blocks.push(block);
        id
    }

    fn update_block(
        &mut self,
        block_id: &str,
        new_data: HashMap<String, Value>,
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

    fn create_version(&mut self) -> u64 {
        let version = self.changes.len() as u64 + 1;
        let snapshot = self.content.blocks.clone();
        self.changes.push(Change {
            block_id: String::new(),
            timestamp: Utc::now(),
            old_data: HashMap::new(),
            new_data: HashMap::from([
                ("version".to_string(), json!(version)),
                ("snapshot".to_string(), json!(snapshot)),
            ]),
        });
        version
    }

    fn restore_version(&mut self, version: u64) -> Result<(), String> {
        if version == 0 || version > self.changes.len() as u64 {
            return Err("Invalid version number".to_string());
        }
        let snapshot = self
            .changes
            .get(version as usize - 1)
            .and_then(|change| change.new_data.get("snapshot"))
            .and_then(|snapshot| snapshot.as_array())
            .ok_or("Invalid snapshot data".to_string())?;

        self.content.blocks = serde_json::from_value(json!(snapshot)).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[allow(dead_code)]
    fn to_editor_js_format(&self) -> serde_json::Value {
        json!({
            "time": self.content.time,
            "blocks": self.content.blocks,
            "version": self.content.version
        })
    }

    fn update_time(&mut self) {
        self.content.time = Utc::now().timestamp_millis();
    }

    #[allow(dead_code)]
    fn attach_block_to_template(
        &mut self,
        block_id: &str,
        template: &ComponentTemplate,
    ) -> Result<(), String> {
        if let Some(block) = self.content.blocks.iter_mut().find(|b| b.id == block_id) {
            let mut new_data = template.default_data.clone();
            new_data.insert("is_attached".to_string(), json!(true));
            new_data.insert("template_id".to_string(), json!(template.id));
            new_data.insert(
                "display_config".to_string(),
                json!(template.default_display_config),
            );

            // Preserve existing data as local overrides
            let local_overrides: HashMap<String, Value> = block
                .data
                .iter()
                .filter(|(k, v)| template.default_data.get(*k) != Some(v))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            if !local_overrides.is_empty() {
                new_data.insert("local_overrides".to_string(), json!(local_overrides));
            }

            self.update_block(block_id, new_data)
        } else {
            Err("Block not found".to_string())
        }
    }

    fn detach_block_from_template(&mut self, block_id: &str) -> Result<(), String> {
        if let Some(block) = self.content.blocks.iter_mut().find(|b| b.id == block_id) {
            let mut new_data = block.data.clone();
            new_data.insert("is_attached".to_string(), json!(false));
            new_data.remove("template_id");

            if let Some(local_overrides) = new_data.remove("local_overrides") {
                if let Some(overrides) = local_overrides.as_object() {
                    for (key, value) in overrides {
                        new_data.insert(key.clone(), value.clone());
                    }
                }
            }

            self.update_block(block_id, new_data)
        } else {
            Err("Block not found".to_string())
        }
    }
}

struct DocumentStore {
    db: Surreal<Client>,
}

impl DocumentStore {
    async fn new(db: Surreal<Client>) -> Result<Self, surrealdb::Error> {
        Ok(Self { db })
    }

    async fn create_document(&self, mut doc: Document) -> Result<Thing, surrealdb::Error> {
        doc.update_time();
        let created: Vec<Document> = self.db.create("documents").content(&doc).await?;
        created
            .first()
            .and_then(|d| d.id.clone())
            .ok_or(surrealdb::Error::Db(surrealdb::error::Db::NoRecordFound))
    }

    async fn get_document(&self, id: &Thing) -> Result<Option<Document>, surrealdb::Error> {
        self.db.select(("documents", id.id.to_raw())).await
    }

    async fn update_document(&self, id: &Thing, mut doc: Document) -> Result<(), surrealdb::Error> {
        doc.update_time();
        self.db
            .update::<Option<Document>>(("documents", id.id.to_raw()))
            .content(&doc)
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    async fn delete_document(&self, id: &Thing) -> Result<(), surrealdb::Error> {
        self.db
            .delete::<Option<Document>>(("documents", id.id.to_raw()))
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    async fn create_version(&self, id: &Thing) -> Result<u64, surrealdb::Error> {
        let mut doc = self
            .get_document(id)
            .await?
            .ok_or(surrealdb::Error::Db(surrealdb::error::Db::NoRecordFound))?;
        let version = doc.create_version();
        self.update_document(id, doc).await?;
        Ok(version)
    }

    #[allow(dead_code)]
    async fn restore_version(&self, id: &Thing, version: u64) -> Result<(), surrealdb::Error> {
        let mut doc = self
            .get_document(id)
            .await?
            .ok_or(surrealdb::Error::Db(surrealdb::error::Db::NoRecordFound))?;
        doc.restore_version(version).map_err(|e| {
            surrealdb::Error::Db(surrealdb::error::Db::InvalidArguments {
                name: "Invalid Version".to_string(),
                message: e,
            })
        })?;
        self.update_document(id, doc).await
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
    ) -> Result<Thing, surrealdb::Error> {
        let created: Vec<ComponentTemplate> = self
            .db
            .create("component_templates")
            .content(template)
            .await?;
        created
            .first()
            .map(|t| t.id.clone())
            .ok_or(surrealdb::Error::Db(surrealdb::error::Db::NoRecordFound))
    }

    #[allow(dead_code)]
    async fn get_template(&self, id: &str) -> Result<Option<ComponentTemplate>, surrealdb::Error> {
        let result: Option<ComponentTemplate> = self.db.select(("component_templates", id)).await?;
        Ok(result)
    }

    async fn update_template(
        &self,
        id: &str,
        updated_template: &ComponentTemplate,
    ) -> Result<(), surrealdb::Error> {
        self.db
            .update::<Option<ComponentTemplate>>(("component_templates", id))
            .content(updated_template)
            .await?;
        Ok(())
    }

    async fn propagate_template_changes(
        &self,
        template_id: &str,
        document_store: &DocumentStore,
    ) -> Result<(), surrealdb::Error> {
        let template = self
            .get_template(template_id)
            .await?
            .ok_or(surrealdb::Error::Db(surrealdb::error::Db::NoRecordFound))?;

        let all_documents: Vec<Document> = document_store.db.select("documents").await?;
        println!(
            "Propagating template changes to {} documents",
            all_documents.len()
        );
        println!("Template: {:?}", template);

for mut doc in all_documents {
        println!("Processing document: {:?}", doc.id);
        let mut updated = false;
        for (block_index, block) in doc.content.blocks.iter_mut().enumerate() {
            println!("  Checking block {} in document", block_index);
            
            let attached_template_id = block.data.get("template_id")
                .and_then(|v| v.get("id"))
                .and_then(|v| v.as_str());
            
            let is_attached = block.data.get("is_attached")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            println!("    Block template_id: {:?}, is_attached: {}", attached_template_id, is_attached);
            
            if let Some(attached_id) = attached_template_id {
                if attached_id == template_id && is_attached {
                    println!("    Block is attached to the updated template");
                    let mut block_updated = false;

                    for (key, value) in &template.default_data {
                        if !block.data.contains_key(key) || 
                           !block.data.get("local_overrides")
                                    .and_then(|v| v.as_object())
                                    .map(|o| o.contains_key(key))
                                    .unwrap_or(false) 
                        {
                            block.data.insert(key.clone(), value.clone());
                            println!("      Updated/Added field: {} = {:?}", key, value);
                            block_updated = true;
                        }
                    }

                    println!("{:?}",template_id);

                    // Ensure core fields are set correctly
                    block.data.insert("is_attached".to_string(), json!(true));
                    block.data.insert("template_id".to_string(), json!({
                        "id": template_id,
                        "tb": "component_templates"
                    }));
                    block.data.insert("display_config".to_string(), json!(template.default_display_config));

                    if block_updated {
                        updated = true;
                        println!("    Block updated");
                    } else {
                        println!("    No changes needed for this block");
                    }
                } else {
                    println!("    Block is not attached to the updated template");
                }
            } else {
                println!("    Block is not associated with any template");
            }
        }
        if updated {
            println!("Updating document in database: {:?}", doc.id);
            document_store
                .update_document(&doc.id.clone().unwrap(), doc)
                .await?;
        } else {
            println!("No updates needed for document: {:?}", doc.id);
        }
    }
    Ok(())
    }
    
}

#[allow(dead_code)]
async fn get_document_as_editor_js(
    store: &DocumentStore,
    id: &Thing,
) -> Result<serde_json::Value, surrealdb::Error> {
    let doc = store
        .get_document(id)
        .await?
        .ok_or(surrealdb::Error::Db(surrealdb::error::Db::NoRecordFound))?;
    Ok(doc.to_editor_js_format())
}

async fn update_template_and_propagate(
    template_store: &ComponentTemplateStore,
    document_store: &DocumentStore,
    template_id: &str,
    updates: &HashMap<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the existing template
    let mut template = template_store
        .get_template(template_id)
        .await?
        .ok_or_else(|| surrealdb::Error::Db(surrealdb::error::Db::NoRecordFound))?;

    println!("Existing template: {:?}", template);

    // Apply the updates
    for (key, value) in updates {
        template.default_data.insert(key.clone(), value.clone());
        println!("Updating field: {} = {:?}", key, value);
    }

    println!("Updated template: {:?}", template);

    // Update the template in the database
    template_store
        .update_template(template_id, &template)
        .await?;
    println!("Template updated in database");

    // Propagate the changes
    template_store
        .propagate_template_changes(template_id, document_store)
        .await?;
    println!("Changes propagated to documents");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<Ws>("127.0.0.1:9000").await?;
    let _ = db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await;
    let _ = db.use_ns("test").use_db("test").await;

    let store = DocumentStore::new(db.clone()).await?;
    let template_store = ComponentTemplateStore::new(db).await?;

    // Create a component template
    let mut author_bio_template = ComponentTemplate {
        id: Thing::from(("component_templates", "ct-001")),
        name: "Author Bio".to_string(),
        default_data: HashMap::from([
            ("name".to_string(), json!("Default Author")),
            ("bio".to_string(), json!("This is a default author bio.")),
            (
                "image_url".to_string(),
                json!("https://example.com/default.jpg"),
            ),
            (
                "social_links".to_string(),
                json!([{"platform": "Twitter", "url": "https://twitter.com/default"}]),
            ),
        ]),
        default_display_config: HashMap::from([
            ("name".to_string(), true),
            ("bio".to_string(), true),
            ("image_url".to_string(), true),
            ("social_links".to_string(), true),
        ]),
    };
    let template_id = template_store.create_template(&author_bio_template).await?;
    println!("Created template with ID: {:?}", template_id);

    // Create a new document with a linked block
    let mut doc = Document::new();
    doc.add_block(
        "header",
        HashMap::from([
            ("text".to_string(), json!("My First Blog Post")),
            ("level".to_string(), json!(1)),
        ]),
    );
    doc.add_block(
        "paragraph",
        HashMap::from([(
            "text".to_string(),
            json!("This is the content of my first blog post."),
        )]),
    );

    // Add a linked authorBio block
    let author_bio_data = HashMap::from([
        ("name".to_string(), json!("John Doe")),
        ("bio".to_string(), json!("Default bio for John Doe")),
        (
            "image_url".to_string(),
            json!("https://example.com/johndoe.jpg"),
        ),
        (
            "social_links".to_string(),
            json!([
                {"platform": "Twitter", "url": "https://twitter.com/johndoe"},
                {"platform": "LinkedIn", "url": "https://linkedin.com/in/johndoe"}
            ]),
        ),
        (
            "display_config".to_string(),
            json!({
                "name": true,
                "bio": true,
                "image_url": false,
                "social_links": true
            }),
        ),
        ("is_attached".to_string(), json!(true)),
        ("template_id".to_string(), json!(Thing::from(("component_templates", "ct-001")))),
    ]);
    let author_bio_id = doc.add_block("authorBio", author_bio_data);

    // Save the document to the database
    let doc_id = store.create_document(doc).await?;
    println!("Created document with ID: {:?}", doc_id);
    println!("id: {:?}", author_bio_id);

    // // Retrieve the document
    let mut retrieved_doc = store.get_document(&doc_id).await?.unwrap();
    // println!("Retrieved document: {:?}", retrieved_doc);

    // Apply a local override to the linked block
    let author_bio_block = retrieved_doc
        .content
        .blocks
        .iter_mut()
        .find(|b| b.id == author_bio_id)
        .unwrap();
    let mut author_bio_data = author_bio_block.data.clone();
    author_bio_data.insert(
        "local_overrides".to_string(),
        json!({
            "bio": "This is a custom bio for John Doe, overriding the template."
        }),
    );
    retrieved_doc.update_block(&author_bio_id, author_bio_data)?;

    // Update the document in the database
    store.update_document(&doc_id, retrieved_doc).await?;

    // Retrieve the updated document and print it
    let updated_doc = store.get_document(&doc_id).await?.unwrap();
    println!("Updated document with local override: {:?}", updated_doc);

    // Detach the block from the template
    let mut detached_doc = updated_doc;
    detached_doc.detach_block_from_template(&author_bio_id)?;

    // Update the document with the detached block
    store.update_document(&doc_id, detached_doc).await?;

    // Retrieve and print the final document
    let final_doc = store.get_document(&doc_id).await?.unwrap();
    println!("Final document with detached block: {:?}", final_doc);

    // Print the change history
    println!("Change history:");
    for (i, change) in final_doc.changes.iter().enumerate() {
        println!("Change {}: {:?}", i + 1, change);
    }
    // Now, let's demonstrate attaching a block to a template
    let mut doc_for_attach = Document::new();
    let new_block_id = doc_for_attach.add_block("newBlock", HashMap::new());
    println!("New block ID: {:?}", new_block_id);

    // Retrieve the template
    let template = template_store.get_template("ct-001").await?.unwrap();

    // Attach the block to the template
    doc_for_attach.attach_block_to_template(&new_block_id, &template)?;

    // Save the document with the attached block
    let doc_id_with_attached = store.create_document(doc_for_attach).await?;

    // Retrieve and print the document with the attached block
    let doc_with_attached = store.get_document(&doc_id_with_attached).await?.unwrap();
    println!("Document with attached block: {:?}", doc_with_attached);

    let updates = HashMap::from([("new_field".to_string(), json!("New default value"))]);
    update_template_and_propagate(&template_store, &store, "ct-001", &updates).await?;

    Ok(())
}


















