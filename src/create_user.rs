use std::io::{self, Write};
use std::fs;
use std::io::{Error, ErrorKind};
use crate::structs::User;
use crate::structs::UserData;
use crate::login::log_in;


pub fn create_user() -> Result< (), io::Error> {

    let mut  username = String::new();
    let mut password_hash = String::new();

    print!("Enter a username : "); io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");

    print!("\nEnter a password : "); io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut password_hash)
        .expect("Failed to read password");

    let new_user =  User {
        username: username.trim().to_string(),
        password_hash: password_hash.trim().to_string(),
    };

    //Read userlist json file, return error if username exists
    let userlist_json = fs::read_to_string("user_list.json")
        .expect("Something went wrong with the userlist");
    let mut users_vector: Vec<User> = serde_json::from_str(&userlist_json).unwrap();
    for every_user in &users_vector {
        if new_user.username == every_user.username {
            let msg = format!("Username already exists!");
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }
    }
    //create json file for user and fill with empty dummy list
    let path = format!("userdata/{}.json", &new_user.username).trim().to_string();
    let path = std::path::Path::new(&path);
    std::fs::File::create(path)
        .expect("Could not create user file");
    let dummy =  UserData { lists : Vec::new()};
    let serialized = serde_json::to_string(&dummy).unwrap();
    fs::write(&path, &serialized).expect("Could not write to user file");

    //Add new user and write to userlist
    users_vector.push(new_user);
    let serialized = serde_json::to_string(&users_vector).unwrap();
    fs::write("user_list.json", &serialized)?;
    
    
    //Call login
    println!("Account created. Login to proceed.\n");
    return log_in();
}