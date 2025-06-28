extern crate backend;
use clap::{Arg, Command};

fn main() {
    let matches = build_cli().get_matches();
    handle_commands(matches);
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
        .arg(Arg::new("id").required(true))
}

fn handle_commands(matches: clap::ArgMatches) {
    match matches.subcommand() {
        Some(("users", sub_matches)) => handle_users_commands(sub_matches),
        _ => unreachable!(),
    }
}

fn handle_users_commands(sub_matches: &clap::ArgMatches) {
    match sub_matches.subcommand() {
        Some(("create", create_matches)) => handle_create_user(create_matches),
        Some(("list", _)) => handle_list_users(),
        Some(("delete", delete_matches)) => handle_delete_user(delete_matches),
        _ => unreachable!(),
    }
}

fn handle_create_user(_matches: &clap::ArgMatches) {
    // TODO: Implement user creation logic
    println!("Creating user...");
}

fn handle_list_users() {
    // TODO: Implement user listing logic
    println!("Listing users...");
}

fn handle_delete_user(_matches: &clap::ArgMatches) {
    // TODO: Implement user deletion logic
    println!("Deleting user...");
}
