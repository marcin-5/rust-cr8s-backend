use crate::repositories::CrateRepository;
use crate::{
    auth,
    models::NewUser,
    models::RoleCode,
    repositories::{RoleRepository, UserRepository},
};
use chrono::{Datelike, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use lettre::message::header::ContentType;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use std::str::FromStr;
use tera::{Context, Tera};

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

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").expect("Cannot load template engine")
}

pub async fn digest_send(email: String, hours_since: i32) {
    let mut c = load_db_connection().await;
    let tera = load_template_engine();
    let crates = CrateRepository::find_since(&mut c, hours_since)
        .await
        .unwrap();

    if crates.len() > 0 {
        let year = Utc::now().year();
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);

        let html_body = tera.render("email/digest.html", &context).unwrap();
        let message = MessageBuilder::new()
            .subject("Cr8s digest")
            .from("Cr8s <noreply@cr8s.com>".parse().unwrap())
            .to(email.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();

        let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load SMTP host from environment");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot load SMTP username from environment");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot load SMTP password from environment");

        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::relay(&smtp_host)
            .unwrap()
            .credentials(credentials)
            .build();
        mailer.send(&message).unwrap();
    }
}
