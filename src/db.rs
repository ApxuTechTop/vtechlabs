use tokio_postgres::{Client, NoTls, Error};
use crate::models::{self, User};

pub async fn create_tables(client: &Client) -> Result<(), Error> {
    client
        .batch_execute("
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name TEXT,
                email TEXT
            )
        ").await?;
    Ok(())
}

pub async fn get_users(client: &Client, start: Option<i64>, end: Option<i64>) -> Result<Vec<User>, Error> {
    let query = if let (Some(start), Some(end)) = (start, end) {
        format!("SELECT * FROM users ORDER BY id LIMIT {} OFFSET {}", end - start, start)
    } else {
        String::from("SELECT * FROM users")
    };

    let stmt = client.prepare(&query).await?;
    let rows = client.query(&stmt, &[]).await?;

    let mut users = Vec::new();
    for row in rows {
        users.push(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        });
    }

    Ok(users)
}

pub async fn create_user(client: &Client, name: &str, email: &str) -> Result<i32, Error> {
    let stmt = client.prepare("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id").await?;
    let row = client.query_one(&stmt, &[&name, &email]).await?;
    let id: i32 = row.get(0);
    Ok(id)
}

pub async fn get_user_by_id(client: &Client, user_id: i32) -> Result<Option<User>, Error> {
    let stmt = client.prepare("SELECT * FROM users WHERE id = $1").await?;
    let row = client.query_opt(&stmt, &[&user_id]).await?;

    match row {
        Some(row) => {
            let user = User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
            };
            Ok(Some(user))
        },
        None => Ok(None)
    }
}

pub async fn delete_user(client: &Client, user_id: i32) -> Result<(), Error> {
    let stmt = client.prepare("DELETE FROM users WHERE id = $1").await?;
    client.execute(&stmt, &[&user_id]).await?;
    Ok(())
}
