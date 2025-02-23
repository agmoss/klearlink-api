use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub api_key: Uuid,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct InsertUserModel {
    pub username: String,
    pub api_key: Uuid,
    pub role: String,
}

impl InsertUserModel {
    pub fn new(username: String, api_key: Uuid, role: String) -> Self {
        Self {
            username,
            api_key,
            role,
        }
    }
}
