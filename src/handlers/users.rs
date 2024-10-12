use std::fmt::format;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, HttpRequest, Responder};
use sea_orm::*;

use crate::schemas::users;
use crate::service::users::Query;
use super::AppState;

#[post("/")]
async fn create_user(data: web::Json<users::UserCreate>, state: web::Data<AppState>) -> impl Responder {
    let conn = &state.conn;
    let username: String = data.username.clone();
    let email: String = data.username.clone();
    let mut password: String = data.username.clone();
    password.push_str("_hashed");

    Query::create_user(conn , &username, &email, &password).await;

    format!("created")
}