use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = args[2].clone();
    
    // println!("Command: {:?}", command); 

    if command == "hello" {
        println!("Hello {}!!", name);
    } else if command == "bye" {
        println!("Bye {}!!", name);
    } else {
        println!("That's not a valid arg command.");
    }
}