extern crate backend;

use backend::commands::{create_user, delete_user, list_users};
use clap::{value_parser, Arg, Command};

#[tokio::main]
async fn main() {
    let matches = build_cli().get_matches();
    handle_commands(matches).await;
}

fn build_cli() -> Command {
    Command::new("Cr8s")
        .about("Cr8s commands")
        .arg_required_else_help(true)
        .subcommand(build_users_command())
}

fn build_users_command() -> Command {
    Command::new("users")
        .about("Manage users")
        .arg_required_else_help(true)
        .subcommand(build_create_user_command())
        .subcommand(build_list_users_command())
        .subcommand(build_delete_user_command())
}

fn build_create_user_command() -> Command {
    Command::new("create")
        .about("Create a new user")
        .arg_required_else_help(true)
        .arg(Arg::new("username").required(true))
        .arg(Arg::new("password").required(true))
        .arg(
            Arg::new("roles")
                .required(true)
                .num_args(1..)
                .value_delimiter(','),
        )
}

fn build_list_users_command() -> Command {
    Command::new("list").about("List existing users")
}

fn build_delete_user_command() -> Command {
    Command::new("delete")
        .about("Delete user by ID")
        .arg_required_else_help(true)
        .arg(
            Arg::new("id")
                .required(true)
                .value_parser(value_parser!(i32)),
        )
}

async fn handle_commands(matches: clap::ArgMatches) {
    match matches.subcommand() {
        Some(("users", sub_matches)) => handle_users_commands(sub_matches).await,
        _ => unreachable!(),
    }
}

async fn handle_users_commands(sub_matches: &clap::ArgMatches) {
    match sub_matches.subcommand() {
        Some(("create", create_matches)) => handle_create_user(create_matches).await,
        Some(("list", _)) => handle_list_users().await,
        Some(("delete", delete_matches)) => handle_delete_user(delete_matches).await,
        _ => unreachable!(),
    }
}

async fn handle_create_user(_matches: &clap::ArgMatches) {
    let username = _matches
        .get_one::<String>("username")
        .expect("Username is required");
    let password = _matches
        .get_one::<String>("password")
        .expect("Password is required");
    let roles: Vec<String> = _matches
        .get_many::<String>("roles")
        .expect("Roles are required")
        .map(|v| v.to_owned())
        .collect();

    create_user(username.to_owned(), password.to_owned(), roles).await
}

async fn handle_list_users() {
    list_users().await;
}

async fn handle_delete_user(matches: &clap::ArgMatches) {
    delete_user(matches.get_one::<i32>("id").unwrap().to_owned()).await;
}
