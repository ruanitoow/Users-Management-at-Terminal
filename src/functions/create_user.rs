use std::fs::{self};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::{get_input, read_json_file};
use crate::structs::user::User;
use crate::enums::returnstatus::ReturnStatus;

pub fn create_user() -> ReturnStatus {
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
        username: username.trim_end().to_string(),
        age: age,
        id: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32,
    };

    let mut sla = read_json_file().unwrap();

    println!("{:#?}", sla);
    
    sla.push(new_user.clone());
    fs::write("Users.json", serde_json::to_string_pretty(&sla).unwrap().as_bytes()).unwrap();
    
    println!("Created new user successfully!\n\n");

    ReturnStatus::SuccessUser(new_user)
}