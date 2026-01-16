use crate::utils::{get_input, read_json_file};
use crate::structs::user::User;
use crate::enums::returnstatus::ReturnStatus;

pub fn find_user_by_id() -> ReturnStatus {
    let users_file: Vec<User> = read_json_file().unwrap();
    let user_id: u32;

    println!("Type the user id you want to find:");
    user_id = match get_input().trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Couldn't convert user id to a valid number.\n");
            0
        }
    };

    let filtered_user: User;
    let return_variable: ReturnStatus;

    return_variable = match users_file.into_iter().filter(|user| user.id == user_id).collect::<Vec<User>>().into_iter().next() {
        Some(u) => {
            filtered_user = u;

            println!("User found: {:#?}", &filtered_user);
            print!("\n");
            ReturnStatus::SuccessUser(filtered_user)
        },
        None => {
            println!("User not found.\n");
            ReturnStatus::Error
        },
    };

    return_variable
}