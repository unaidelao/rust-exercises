pub fn run() {
    let age: u8 = 17;
    let is_student: bool = false;

    if age >= 18 && is_student {
        println!("You should go to college.");
    } else if age >= 18 && !is_student {
        println!("You better go home to do homeworks.");
    } else {
        println!("You are under 18 years old.");
    }

    // Rust does not have ternary operator.
    let has_enough_age = if age >= 18 { true } else { false };
    println!("Has enough age: {}", has_enough_age);
}