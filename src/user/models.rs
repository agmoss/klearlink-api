use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub api_key: String, // TODO: Make this a UUID
}

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct InsertUsers {
    pub username: String,
    pub api_key: String,
}
