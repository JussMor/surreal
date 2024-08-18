use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn function_connect_to_surreal() -> surrealdb::Result<Surreal<Client>> { 
  let db = Surreal::new::<Ws>("127.0.0.1:9000").await?;

  db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

  db.use_ns("upbase").use_db("upbase").await?;

  Ok(db)
}
