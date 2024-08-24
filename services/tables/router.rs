use ntex::web::{ServiceConfig, scope};


use super::tables;

pub fn config(cfg:&mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/tables")
                .service(tables::create_db_table));
}


// #[derive(Debug, Serialize, thiserror::Error)]
// struct UpbaseError(SurrealError);

// impl WebResponseError for UpbaseError {}

// impl std::fmt::Display for UpbaseError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self.0)
//     }
// }
