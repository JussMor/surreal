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
mod errors;


#[derive(Clone)]
pub struct AppState {
    db: Arc<Surreal<Client>>,
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
    
    



    let state = AppState {
        db: db,
    };
    
     
     HttpServer::new(move || {
         App::new()
         .state(state.clone())
         .wrap(Cors::new().allowed_origin("*").finish())
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
             .configure(services::blocks::router::config)
             .configure(services::tables::router::config)
     })
     .bind("127.0.0.1:3030")?
     .run()
     .await
}
