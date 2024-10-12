use serde::{Deserialize, Serialize};
use uuid::Uuid;
use datetime::LocalDateTime;

#[derive(Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String
}