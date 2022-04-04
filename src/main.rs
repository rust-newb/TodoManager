
/*  
    To add: regex to verify inputs, hashing function to generate hash
    To do: Implement error handling, and split code in modules 
*/

use std::io::{self, Write};
use std::fs;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};


#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password_hash: String,
}

// list of listnames and tasklists
#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    lists: Vec<(String, Vec<(String, bool)>)>
}




fn user_interface(user :User) {

    //load instance of Userdata from user's json file
    let user_filename = "userdata/".to_owned() + &user.username + ".json";
    let userdata_json = fs::read_to_string(&user_filename)
        .expect("Something went wrong with the userfile");
    let mut userdata: UserData = serde_json::from_str(&userdata_json).unwrap();
 
    loop {
        println!("Your lists are : ");                                      //print names of lists
        for lists in &userdata.lists {
            println!("\t{}", lists.0);
        }

        //main CI interface
        println!("Enter an operation(1,2,3,4) and listname, separated by space:\n\t 
            1. View list\n\t
            2. Create new list\n\t
            3. Update list\n\t
            4. Delete list\n\t
            5. Logout");

        let mut response = String::new();

        io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
        
        let option = &response.remove(0);
        let list_name = response.trim().to_string();


        match option {
            '1' =>  {
                //Find a list name and print
                if let Some(pos) = userdata.lists.iter().position(|x| *x.0 == list_name) {
                    println!("List {}: \n\n {:?}", list_name,userdata.lists[pos]);
                }
                else {println!("No such list {}", list_name)}

            },
            '2' =>  {
                //Add new empty list
                let newlist =  (list_name.to_string(), Vec::new());
                userdata.lists.push(newlist);
                println!("List {} created successfully", list_name);
            },
            '3' => {
                //Find list
                if let Some(pos) = userdata.lists.iter().position(|x| *x.0 == list_name) {
                    let mut tasks = &mut userdata.lists[pos].1;

                    //second interface
                    println!("Enter an operation(1,2,3) and task name, separated by space:\n\t
                        1. Add task\n\t
                        2. Delete task\n\t
                        3. Mark task as done\n\t");
        
                    let mut response = String::new();
        
                    io::stdin()
                    .read_line(&mut response)
                    .expect("Failed to read line");
                
                    let option = response.remove(0);
                    let task_name = response.trim().to_string();

                    match option {
                        '1' => {
                            // Add new task by name
                            tasks.push((task_name, false));
                        },
                        '2' => {
                            // Delete task by name
                            if let Some(pos) = tasks.iter().position(|x| *x.0 == task_name) {
                                tasks.remove(pos);
                                println!("Task {} deleted successfully", task_name);
                            }
                            else {println!("No such task {}", task_name);}
                        },
                        '3' => {
                            //Mark task done by name
                            if let Some(pos) = tasks.iter().position(|x| *x.0 == task_name) {
                                tasks[pos].1 = true;
                                println!("Task {} marked as done", task_name);
                            }
                            else {println!("No such task {}", task_name);}                           
                        },
                        _ => println!("Please try again"),
                    }
                }
                else {println!("No such list {}", list_name)}
                
            }
            '4' => {
                //Find list and delete
                if let Some(pos) = userdata.lists.iter().position(|x| *x.0 == list_name) {
                    userdata.lists.remove(pos);
                    println!("List {} deleted successfully", list_name);
                }
                else {println!("No such list {}", list_name)}
            }

            //logout to main
            '5' => {return;}

            _ => println!("Please try again"),
        }
        //write any updates back to users json file
        let serialized = serde_json::to_string(&userdata).unwrap();
        fs::write(&user_filename, &serialized).expect("Could not write to user file");
    }
}

fn create_user() -> Result< (), io::Error> {

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