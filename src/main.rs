mod models;

use postgres::types::Type;
use postgres::{Client, Error, NoTls};
use std::env;
use warp::Filter;
use serde::Deserialize;

fn select(client: &mut Client) -> Vec<models::User> {
    let mut users = vec![];
    for row in client.query("SELECT * FROM users", &[]).expect("") {
        let user = models::User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        };
        users.push(user);
        //println!("Record: {:?}\n", &user);
    }
    return users;
}

fn insert(client: &mut Client, name: String, email: String) -> Vec<models::User> {
    let statement = client.prepare_typed(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[Type::VARCHAR, Type::VARCHAR],
    ).unwrap();
    let res = client.execute(&statement, &[&name, &email]).expect("Couldn't insert user");

    print!("Result while INSERT -> {}\n", &res);
    select(client)
}

fn drop(client: &mut Client, user_id: i32) -> Result<(), Error> {
    let statement = client.prepare_typed("DELETE FROM users WHERE id = $1", &[Type::INT4])?;

    let res = client.execute(&statement, &[&user_id])?;

    print!("Result while drop -> {}\n", &res);

    Ok(())
}

fn update(client: &mut Client, user_id: i32, name: String, email: String) -> Result<(), Error> {
    let statement = client.prepare_typed(
        "UPDATE users SET name, email = $2, $3  WHERE id = $1",
        &[Type::INT4, Type::VARCHAR, Type::VARCHAR],
    )?;

    let res = client.execute(&statement, &[&user_id, &name, &email])?;

    print!("Result while update -> {}\n", &res);

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Request {
    pub name: String,
    pub email: String,
}

#[tokio::main] // 2.
async fn main() {
    //dotenv().ok();
    let host = env::var("POSTGRES_HOST").expect("");
    let user = env::var("POSTGRES_USER").expect("");
    let passwd = env::var("POSTGRES_PASSWORD").expect("");
    println!("{}", host);
    //"postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/users_database"
    let db_url = format!("postgres://{user}:{passwd}@{host}:5432/postgres");
    println!("{}", db_url);
    let mut client = Client::connect(&db_url, NoTls).expect("Not connected to sql");

    insert(&mut client, String::from("apxu"), String::from("mail"));
    insert(&mut client, String::from("polyak"), String::from("mailru"));
    select(&mut client);

    println!("start server 2\n");
    let db_url = format!("postgres://{user}:{passwd}@{host}:5432/postgres");
    let get_users = warp::path!("users").and(warp::get()).map(move || {
        let mut client = Client::connect(&db_url, NoTls).expect("Not connected to sql");
        warp::reply::json(&select(&mut client))
    });
    let db_url = format!("postgres://{user}:{passwd}@{host}:5432/postgres");
    let create_user = warp::path!("users").and(warp::put()).and(warp::body::json()).map(move |request: Request| {
        let mut client = Client::connect(&db_url, NoTls).expect("Not connected to sql");
        warp::reply::json(&insert(&mut client, request.name, request.email))
    });
    let db_url = format!("postgres://{user}:{passwd}@{host}:5432/postgres");
    let delete_user = warp::path!("users" / i32).and(warp::delete()).map(move |id| {
        let mut client = Client::connect(&db_url, NoTls).expect("Not connected to sql");
        drop(&mut client, id).unwrap();
        warp::reply::json(&1)
    });
    let healthcheck = warp::path!("check").map(warp::reply);
    
    let routes = get_users.or(create_user).or(delete_user).or(healthcheck);
    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
