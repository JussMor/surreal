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

    // Upbase Code
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