use std::fmt::format;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, HttpRequest, Responder, Result, Error};
use sea_orm::*;
use serde_json::json;

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

#[get("/")]
async fn get_all_users(req: HttpRequest, state: web::Data<AppState>) -> web::Json<Vec<serde_json::Value>> {
    let conn = &state.conn;
    let params = web::Query::<users::Params>::from_query(req.query_string()).unwrap();
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(5);
    let (users, num_pages) = Query::get_all_users(conn, page, page_size)
        .await
        .expect("Cannot find users in page");
    println!("{:?}", users);
    web::Json(vec!(json!(users)))
}