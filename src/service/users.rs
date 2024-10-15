use uuid::Uuid;
use std::vec::Vec;

use crate::models::entities::{users, users::Entity as Users};
use crate::schemas::users::UserGet;
use crate::utils::hash;
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn create_user(db: &DbConn, username: &String, email: &String, password: &String) -> String {
        let user_id: Uuid = Uuid::new_v4();
        let hashed_password: String = hash(password.as_bytes()).await;
        let new_user = users::ActiveModel {
            uuid: ActiveValue::Set(user_id),
            username: ActiveValue::Set(username.to_owned()),
            email: ActiveValue::Set(email.to_owned()),
            password_hash: ActiveValue::Set(hashed_password),
            ..Default::default()
        };
        let user_res = Users::insert(new_user).exec(db).await.unwrap();

        match user_res {
            _ => {format!("inserted")},
        }
    }

    pub async fn get_all_users(db: &DbConn, page: u64, page_size: u64) -> Result<(Vec<UserGet>, u64), DbErr> {
        let paginator = Users::find()
            .order_by_asc(users::Column::Uuid)
            // .into_json()
            .paginate(db, page_size);
        let num_pages = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| { 
            let mut users: Vec<UserGet> = Vec::new();
            for user in p {
                users.push(UserGet::from_model(user));
            }
            (users, num_pages)
        })
    }

    pub async fn get_one_user(db: &DbConn, user_id: Uuid) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(user_id).one(db).await
    }
}