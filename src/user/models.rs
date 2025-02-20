use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub api_key: Uuid,
}

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct InsertUsers {
    pub username: String,
    pub api_key: Uuid,
}
