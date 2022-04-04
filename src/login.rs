use std::io::{self, Write};
use std::fs;
use std::io::{Error, ErrorKind};
use crate::structs::User;
use crate::user_interface::user_interface;

pub fn log_in () -> Result< (), io::Error> {

    let mut  username = String::new();
    let mut password_hash = String::new();

    print!("Enter your username : "); io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");

    print!("\nEnter your password : "); io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut password_hash)
        .expect("Failed to read password");

    let new_login =  User {
        username: username.trim().to_string(),
        password_hash: password_hash.trim().to_string(),
    };

    //Read userlist and authenicate login, calling user_interface with login
    let userlist_json = fs::read_to_string("user_list.json")
        .expect("Something went wrong with the userlist");
    let users_vector: Vec<User> = serde_json::from_str(&userlist_json).unwrap();

    for every_user in &users_vector {
        if new_login.username == every_user.username && new_login.password_hash == every_user.password_hash {
            println!("Login successful. Welcome {}", new_login.username);
            user_interface(new_login);
            return Ok(());
        }
    }
    let msg = format!("Username, password do not match");
        return Err(Error::new(ErrorKind::InvalidData, msg));
}