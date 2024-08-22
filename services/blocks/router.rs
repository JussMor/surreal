use ntex::web::{ServiceConfig, scope};


use super::blocks;

pub fn config(cfg:&mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/blocks")
                .service(blocks::create_block));
}