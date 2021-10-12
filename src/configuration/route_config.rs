use actix_web::web;
use actix_cors::Cors;

pub fn get_cors() -> Cors {
    Cors::permissive().max_age(3600)
}

// Endpoints registration config
pub fn routes(config: &mut web::ServiceConfig) {
    // Only imports of Endpoints
}
