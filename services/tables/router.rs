use ntex::web::{ServiceConfig, scope};


use super::tables;

pub fn config(cfg:&mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/tables")
                .service(tables::get_db_tables));
}