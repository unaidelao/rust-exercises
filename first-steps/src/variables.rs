// Variables hold primitive data or references to data.
// Variables are inmutable by default.
// Rust is a block-scoped language.

pub fn run() {
    // Inmutable variable.
    let city = "Madrid";
    // Mutable variable
    let mut num = 1;

    println!("The city is {}. It has {} Rust team.", city, num);

    num = 2;

    println!("The city is {}, but now it has {} Rust teams.", city, num);

    // Constant declaration. When declaring a constant, defining the type is necessary.
    const QUANTITY: i8 = 5;

    println!("The quantity is {}.", QUANTITY);

    // Multiple variables declaration + initialization.
    let (bird_name, total_birds) = ("Abubilla", 450);
    println!("Total of {} birds: {}.", bird_name, total_birds);
}