use std::{collections::HashMap};
use crate::utils::get_input;
use crate::structs::user::User;
use crate::enums::returnstatus::ReturnStatus;

pub fn create_user(users_hash_map: &mut HashMap<u32, User>) -> ReturnStatus {
    let username: String;
    let age: i8;

    println!("Type the user name:");
    username = get_input();

    loop {
        let age_demo: i8;
        println!("Type the user age:");

        age_demo = match get_input().trim().parse() {
            Ok(n) => n,
            Err(_) => 0
        };

        if age_demo <= 0 || age_demo > 100 {
            println!("Age must be a number higher than 0 and lower than 100.");
            continue;
        } else {
            age = age_demo;
            break;
        }
    }

    let new_user = User {
        username: username,
        age: age,
        id: rand::random_range(1000000000..=2000000000)
    };

    println!("Created new user successfully!\n\n");
    users_hash_map.insert(new_user.id, new_user.clone());

    ReturnStatus::SuccessUser(new_user)
}