use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use uuid::Uuid;

use super::models::UserModel;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UserDto {
    #[validate(min_length = 2)]
    pub username: String,
    pub api_key: Uuid,
    #[validate(min_length = 5)]
    pub role: String,
}


impl From<UserModel> for UserDto {
    fn from(consumer_credit: UserModel) -> Self {
        UserDto {
            username: consumer_credit.username,
            api_key: consumer_credit.api_key,
            role: consumer_credit.role,
        }
    }
}
