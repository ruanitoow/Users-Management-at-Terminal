pub mod functions;
pub mod structs;
pub mod utils;
pub mod enums;

use std::{collections::HashMap};
use crate::{
    enums::returnstatus::ReturnStatus, functions::{create_user::create_user, delete_user::delete_user, find_user_by_id::find_user_by_id, list_all_users::list_all_users}, structs::user::User, utils::get_input
};

fn main() {
    let mut users: HashMap<u32, User> = HashMap::new();

    loop {
        println!("Hello, this is the User Management Service. Please select an option below: ");
        println!("(1) Create new user.");
        println!("(2) Find user by id.");
        println!("(3) List all users.");
        println!("(4) Delete an user.");
        println!("(0) Exit.");

        let user_choice = get_input();

        let number_choice: u8 = match user_choice.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Couldn't read user input\n");
                continue;
            },
        };
        
        let return_status: ReturnStatus;
        match number_choice {
            0 => break,
            1 => {
                return_status = create_user(&mut users);
            },
            2 => {
                return_status = find_user_by_id(&mut users);
            },
            3 => {
                return_status = list_all_users(&mut users);
            },
            4 => {
                return_status = delete_user(&mut users);
            },
            _ => continue
        }

        let _return_status: User = match return_status {
            ReturnStatus::SuccessUser(user) => user,
            ReturnStatus::Success => continue,
            ReturnStatus::Error => continue,
            ReturnStatus::Null => continue
        };
    }

    println!("All created users:\n{:?}", users);
}