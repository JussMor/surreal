use ntex::web::{scope, ServiceConfig};

use super::templates;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/v1/templates")
            .service(templates::create_template)
            .service(templates::get_all_templates),
    );
}
