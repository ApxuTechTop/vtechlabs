use super::*;
use actix_web::{web, Error, HttpResponse};
use serde_json::json;
use serde::Deserialize;
use actix_web::{delete, get, post, put,};


#[derive(Deserialize)]
pub struct GetUsersQuery {
    query_string: Option<String>,
    page: Option<usize>,
    items_per_page: Option<usize>,
}

#[get("/users")]
pub async fn get_users(conn: web::Data<DbConn>, query: web::Query<GetUsersQuery>) -> Result<HttpResponse, Error> {
    // let mut stmt = user::Entity::find();
    let users = user::Entity::find().all(conn).await.unwrap();
    Ok(HttpResponse::Ok().json(users))
}

pub async fn create_user(user: web::Json<user::Model>) -> Result<HttpResponse, Error> {
    let new_user = user::Entity::insert()
        .set(User::name.eq(&user.name))
        .set(User::email.eq(&user.email))
        .exec()
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(new_user))
}

// pub async fn get_user_by_id(path: web::Path<(i32,)>) -> HttpResponse {
//     let user_id = path.0;
//     let user = User::find_by_id(user_id).one().await.unwrap();
//     HttpResponse::Ok().json(user)
// }

// pub async fn delete_user(path: web::Path<(i32,)>) -> HttpResponse {
//     let user_id = path.0;
//     let deleted_user = User::delete().filter(User::id.eq(user_id)).exec().await.unwrap();
//     HttpResponse::Ok().json(json!({ "deleted": deleted_user }))
// }
