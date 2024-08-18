use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;


#[derive(Debug, Serialize, Deserialize)]
struct Name {
    first: String,
    last: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Marketing {
    #[allow(dead_code)]
    marketing: bool,
    name: Name,
}



pub async fn query_marketing(db: &Surreal<Client>) -> surrealdb::Result<Vec<Marketing>> {
    let mut all_result = db
        .query("SELECT marketing, name FROM type::table($table)")
        .bind(("table", "person"))
        .await?;

    let all: Vec<Marketing> = all_result.take(0)?;
    Ok(all)
}