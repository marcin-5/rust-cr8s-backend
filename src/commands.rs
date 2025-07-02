use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{
    models::NewUser,
    repositories::{RoleRepository, UserRepository},
};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from environment");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let salt = SaltString::generate(OsRng);
    let argon2 = argon2::Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    let password = password_hash.to_string();

    let new_user = NewUser { username, password };
    let user = UserRepository::create_with_roles(&mut c, new_user, role_codes)
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
