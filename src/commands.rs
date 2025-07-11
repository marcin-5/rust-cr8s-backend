use crate::{
    auth,
    models::NewUser,
    models::RoleCode,
    repositories::{RoleRepository, UserRepository},
};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use std::str::FromStr;

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from environment");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let password_hash = auth::hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        password: password_hash,
    };
    let role_enums = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v.as_str()).unwrap())
        .collect();
    let user = UserRepository::create_with_roles(&mut c, new_user, role_enums)
        .await
        .unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("Roles assigned {:?}", roles);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;
    let users = UserRepository::find_with_roles(&mut c).await.unwrap();

    for (i, user) in users.iter().enumerate() {
        println!("{:05} {:?}", i + 1, user);
        if i < users.len() - 1 {
            println!("-----");
        }
    }
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
    UserRepository::delete(&mut c, id).await.unwrap();
}
