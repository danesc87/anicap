use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into,
    update,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use chrono::NaiveDateTime;

use super::schema::app_user;
use super::schema::app_user::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct AppUser {
    pub id: i16,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub register_at: NaiveDateTime
}
