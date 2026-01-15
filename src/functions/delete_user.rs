use std::{collections::HashMap};
use crate::utils::get_input;
use crate::structs::user::User;
use crate::enums::returnstatus::ReturnStatus;

pub fn delete_user(users_hash_map: &mut HashMap<u32, User>) -> ReturnStatus {
    let user_id: u32;

    println!("Type the user id you want to delete:");
    user_id = match get_input().trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Couldn't convert user id to a valid number.\n");
            0
        }
    };

    users_hash_map.remove(&user_id);

    println!("User deleted successfully!\n");

    ReturnStatus::Success
}