use uuid::Uuid;

use crate::models::entities::{users, users::Entity as Users};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn create_user(db: &DbConn, username: &String, email: &String, password_hash: &String) -> String {
        let user_id: Uuid = Uuid::new_v4();
        let new_user = users::ActiveModel {
            uuid: ActiveValue::Set(user_id),
            username: ActiveValue::Set(username.to_owned()),
            email: ActiveValue::Set(email.to_owned()),
            password_hash: ActiveValue::Set(password_hash.to_owned()),
            ..Default::default()
        };
        let user_res = Users::insert(new_user).exec(db).await.unwrap();

        match user_res {
            _ => {format!("inserted")},
        }
    }
}