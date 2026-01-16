use std::{error::Error, fs::File, io::{self, BufReader}};

use crate::structs::user::User;

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read user input.\n\n");

    input
}

pub fn read_json_file() -> Result<Vec<User>, Box<dyn Error>> {
    let file = File::open("Users.json")?;
    let buf_reader = BufReader::new(file);
    let users: Vec<User>;

    match serde_json::from_reader(buf_reader) {
        Ok(u) => users = u,
        Err(_) => users = Vec::new(),
    }

    Ok(users)
}