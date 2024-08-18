

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Thing, Value};
use surrealdb::Surreal;


#[derive(Debug, Serialize, Deserialize)]
struct Name {
    first: String,
    last: String,
}

#[derive(Debug, Serialize)]
struct Person {
    title: String,
    name: Name,
    marketing: bool,
}

#[derive(Debug, Serialize, )]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}


#[derive(Debug, Deserialize)]
struct Group {
    #[allow(dead_code)]
    marketing: bool,
    count: u64,
}


#[derive(Debug, Deserialize)]
struct GroupMarketing {
    #[allow(dead_code)]
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Marketing {
    #[allow(dead_code)]
    marketing: bool,
    name: Name,
}



#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:9000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("upbase").use_db("upbase").await?;

    // Create a new person with a random id
    // let created: Vec<Record> = db
    //     .create("person")
    //     .content(Person {
    //         title: "Founder & CEO",
    //         name: Name {
    //             first: "Junior Moreira",
    //             last: "Morgan Hitchcock",
    //         },
    //         marketing: true,
    //     })
    // .await?;
    // dbg!(created);

    // let people: Vec<Record> = db.select("person").await?;
    // dbg!(people);

   // Perform a custom advanced query
    // let mut groups_result = db
    //     .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
    //     .bind(("table", "person"))
    //     .await?;
    
    // Deserialize the query result into a Vec<Group>
    // let groups: Vec<Group> = groups_result.take(0)?;
    // println!("{:?}", groups);


    let mut all_result = db
        .query("SELECT marketing, name FROM type::table($table)")
        .bind(("table", "person"))
        .await?;

    let all: Vec<Marketing> = all_result.take(0)?;
    dbg!(all);

    Ok(())
}

