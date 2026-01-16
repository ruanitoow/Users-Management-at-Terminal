pub mod functions;
pub mod structs;
pub mod utils;
pub mod enums;

use std::{fs::{OpenOptions}, io::{self}};
use crate::{
    functions::{create_user::create_user, delete_user::delete_user, find_user_by_id::find_user_by_id, list_all_users::list_all_users}, utils::{get_input, read_json_file}
};

fn main() -> io::Result<()> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("Users.json")?;

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
        
        match number_choice {
            0 => break,
            1 => {
                create_user();
            },
            2 => {
                find_user_by_id();
            },
            3 => {
                list_all_users();
            },
            4 => {
                delete_user();
            },
            _ => continue
        }
    }

    println!("All created users:\n{:#?}", read_json_file().unwrap());

    Ok(())
}