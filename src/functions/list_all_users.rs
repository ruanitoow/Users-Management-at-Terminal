use crate::structs::user::User;
use crate::enums::returnstatus::ReturnStatus;
use crate::utils::read_json_file;

pub fn list_all_users() -> ReturnStatus {
    let users_file: Vec<User> = read_json_file().unwrap();

    println!("Listing all users:\n{:#?}", users_file);
    print!("\n\n");

    ReturnStatus::Success
}