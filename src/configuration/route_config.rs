use actix_web::web;
use actix_cors::Cors;

pub fn get_cors() -> Cors {
    Cors::permissive().max_age(3600)
}

// Endpoints registration config
pub fn routes(config: &mut web::ServiceConfig) {
    use crate::controller::{
        app_user_controller::{
            register,
            login
        },
        user_serie_controller::{
            insert_serie,
            get_series,
            update_serie
        }
    };

    config
        .service(register)
        .service(login)
        .service(insert_serie)
        .service(get_series)
        .service(update_serie);
}
