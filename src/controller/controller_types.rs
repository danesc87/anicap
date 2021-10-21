use actix_web::HttpResponse;

use crate::utils::error_mapper::ServerError;

pub type ServerResponse = Result<HttpResponse, ServerError>;
