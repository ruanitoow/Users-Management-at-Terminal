use std::io;

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read user input.\n\n");

    input
}