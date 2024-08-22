




// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {

//     let _db = db::connection::function_connect_to_surreal().await?;

//     pretty_env_logger::init();

//     let routes = router(db);

//     warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

//     Ok(())
// }

// pub fn router(db: Surreal<Client>,) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone  {
//         let db_filter = warp::any().map(move || db.clone());

//         let routes =
//             warp::path("echo")
//             .and(warp::ws())
//             .and(db_filter)
//             .map(|ws: warp::ws::Ws, db_filter| {
//                 ws.on_upgrade(move |socket| handle_websocket(socket, db_filter))
//             });

//         routes
// }

// pub async fn handle_websocket(ws: WebSocket, db_filter: Surreal<Client>) {
//     let (tx, mut rx) = ws.split();

//     let all = match service::persona::query_marketing(&db_filter).await {
//         Ok(data) => data,
//         Err(e) => {
//             eprintln!("Query error: {:?}", e);
//             return;
//         }
//     };

//     let all_json = match serde_json::to_string(&all) {
//         Ok(json) => {
//             let wrapped_json = serde_json::json!({
//                 "action": "get_marketing_data",
//                 "data": json,
//             });
//             serde_json::to_string(&wrapped_json).unwrap()
//         }
//         Err(e) => {
//             eprintln!("Serialization error: {:?}", e);
//             return;
//         }
//     };

//     let mut tx = tx.sink_map_err(|e| {
//         eprintln!("WebSocket send error: {:?}", e);
//     });

//     while let Some(result) = rx.next().await {
//         match result {
//             Ok(msg) => {
//                 if let Ok(text) = msg.to_str() {
//                     if let Ok(action) = serde_json::from_str::<Action>(text) {
//                         if action.action == "get_marketing_data" {
//                             if let Err(e) = tx.send(Message::text(all_json.clone())).await {
//                                 eprintln!("WebSocket send error: {:?}", e);
//                             }
//                         }
//                     }
//                 }
//             }
//             Err(e) => {
//                 eprintln!("WebSocket error: {:?}", e);
//             }
//         }
//     }
// }

// #[derive(Deserialize)]
// struct Action {
//     action: String,
// }
