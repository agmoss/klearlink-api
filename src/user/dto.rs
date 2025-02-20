use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use uuid::Uuid;

use super::models::InsertUsers;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UserDto {
    #[validate(min_length = 2)]
    pub username: String,
    pub api_key: Uuid,
}

impl UserDto {
    pub fn to_insert_user(&self) -> InsertUsers {
        InsertUsers {
            username: self.username.to_string(),
            api_key: self.api_key.clone(),
        }
    }
}
