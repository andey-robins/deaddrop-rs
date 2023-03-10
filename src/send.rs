use std::io::{self, BufRead};

use crate::db::{users, messages};

pub fn send_message(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        panic!("User not recognized");
    }

    let message = get_user_message();

    messages::save_message(message, user);
}

fn get_user_message() -> String {
    println!("Enter your message: ");
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}