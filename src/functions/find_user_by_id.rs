use std::{collections::HashMap};
use crate::utils::get_input;
use crate::structs::user::User;
use crate::enums::returnstatus::ReturnStatus;

pub fn find_user_by_id(users_hash_map: &mut HashMap<u32, User>) -> ReturnStatus {
    let user_id: u32;

    println!("Type the user id you want to find:");
    user_id = match get_input().trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Couldn't convert user id to a valid number.\n");
            0
        }
    };

    let filtered_user = users_hash_map.get(&user_id);

    let return_variable: ReturnStatus = match filtered_user {
        Some(user) => {
            println!("User found: {:?}", &user);
            print!("\n");
            ReturnStatus::SuccessUser(user.clone())
        },
        None => ReturnStatus::Error
    };

    return_variable
}