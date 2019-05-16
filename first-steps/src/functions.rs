pub fn run() {
    greeting("Aloha", "Pantoja");

    // Bind function values to variables
    let sum_result = add(10, 5);
    println!("Sum result 10 + 5 = {}", sum_result);

    // Closure
    let num3: i32 = 50;
    let add_closure = |num1: i32, num2: i32| num1 + num2 + num3;

    println!("Closure sum: {}", add_closure(2, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, this is a Rust greeting function!", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    // Return value always without semicolon
    num1 + num2
}