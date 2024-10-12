use serde::{Deserialize, Serialize};
use uuid::Uuid;
use datetime::LocalDateTime;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct UserGet {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub registration_date: NaiveDate,
    pub is_admin: bool,
    pub is_confirmed: bool
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}