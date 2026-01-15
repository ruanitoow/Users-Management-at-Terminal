use std::{collections::HashMap};
use crate::structs::user::User;
use crate::enums::returnstatus::ReturnStatus;

pub fn list_all_users(users_hash_map: &mut HashMap<u32, User>) -> ReturnStatus {
    println!("Listing all users:\n{:?}", users_hash_map);
    print!("\n\n");

    ReturnStatus::Success
}