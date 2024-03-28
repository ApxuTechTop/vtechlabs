use actix_web::{web, HttpResponse};
use serde_json::json;
use sea_orm::entity::prelude::*;
use sea_orm::query::prelude::*;
use sea_orm::DatabaseTrait;

use crate::models::User;

pub async fn get_users() -> HttpResponse {
    let users = User::find().all().await.unwrap();
    HttpResponse::Ok().json(users)
}

pub async fn create_user(user: web::Json<User>) -> HttpResponse {
    let new_user = User::insert()
        .set(User::name.eq(&user.name))
        .set(User::email.eq(&user.email))
        .exec()
        .await
        .unwrap();
    HttpResponse::Ok().json(new_user)
}

pub async fn get_user_by_id(path: web::Path<(i32,)>) -> HttpResponse {
    let user_id = path.0;
    let user = User::find_by_id(user_id).one().await.unwrap();
    HttpResponse::Ok().json(user)
}

pub async fn delete_user(path: web::Path<(i32,)>) -> HttpResponse {
    let user_id = path.0;
    let deleted_user = User::delete().filter(User::id.eq(user_id)).exec().await.unwrap();
    HttpResponse::Ok().json(json!({ "deleted": deleted_user }))
}
