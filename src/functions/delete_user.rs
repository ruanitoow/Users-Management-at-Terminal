use std::fs::{self};

use crate::structs::user::{User};
use crate::utils::{get_input, read_json_file};
use crate::enums::returnstatus::ReturnStatus;

pub fn delete_user() -> ReturnStatus {
    let users_file: Vec<User> = read_json_file().unwrap();

    let user_id: u32;

    println!("Type the user id you want to delete:");

    user_id = match get_input().trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Couldn't convert user id to a valid number.\n");
            0
        }
    };

    match users_file.iter().filter(|user| user.id == user_id).collect::<Vec<&User>>().into_iter().next() {
        Some(_) => {
            let updated_users: Vec<User> = users_file.into_iter().filter(|user| user.id != user_id).collect::<Vec<User>>();
            fs::write("Users.json", serde_json::to_string_pretty(&updated_users).unwrap()).unwrap();

            println!("User deleted successfully!\n");
        },
        None => println!("User was not found. No user got deleted.")
    }

    ReturnStatus::Success
}