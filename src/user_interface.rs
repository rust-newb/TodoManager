use std::io;
use std::fs;
use crate::structs::User;
use crate::structs::UserData;

pub fn user_interface(user :User) {

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
                    let tasks = &mut userdata.lists[pos].1;

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