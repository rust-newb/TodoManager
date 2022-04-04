
/*  
    To add: regex to verify inputs, hashing function to generate hash
    To do: Implement error handling, in main
*/
mod structs;
mod login;
mod user_interface;
mod create_user;

use std::io;
use crate::login::log_in;
use crate::create_user::create_user;

fn main() {

    loop {
        println!("Welcome to To-do manager\n");

        println!("Are you a new user? (Y/N)");

        let mut response = String::new();

        io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
        
        match response.trim() {
            "Y" | "y" =>  {create_user();},
            "N" | "n" =>  {log_in();},
             _ => println!("Please try again (Y/N)"),
        }

    }
}