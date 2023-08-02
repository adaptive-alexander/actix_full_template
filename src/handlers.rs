use actix_web::{route, web, HttpResponse};
use serde_json::json;

/// REST endpoint for health check
#[route("/health", method = "GET")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!("I'm healthy."))
}

/// Register services
pub fn register(config: &mut web::ServiceConfig) {
    config
        .service(health);
}