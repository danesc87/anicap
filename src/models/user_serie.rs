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

use super::schema::user_serie;
use super::schema::user_serie::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserSerie {
    pub id: i16,
    pub user_id: i16,
    pub serie: String,
    pub score: f32,
    pub chapter: i16
}
