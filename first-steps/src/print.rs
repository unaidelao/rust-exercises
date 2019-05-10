pub fn run() {
    // Basic printing
    println!("This is the run() function from print.rs");

    // Print formatting
    println!("My name is {} and I'm from {}.", "Unai", "Spain");

    println!("Today is {0}, and I started learning {1}. It's a great {0}.", "Friday", "Rust");

    println!("{2} has {1} horses, but {0} has {1} more.", "John", 2, "Elisa");

    println!("I'm learning {language} while using {IDE}.", language = "Rust", IDE = "VSCode");

    // Number formatting
    println!("2019 in Binary: {:b}", 2019);
    println!("2019 in Hexadecimal: {:x}", 2019);
    println!("2019 in Octal: {:o}", 2019);

    // Debug trait
    println!("Debug trait: {:?}", ("Pontiac", 35, true, "Europe", false, -4));

    // Math operators
    println!("Adition: 5 + 2 = {}", 5 + 2);
    println!("Substraction: 5 - 2 = {}", 5 - 2);
    println!("Multiplication: 5 * 2 = {}", 5 * 2);
    println!("Division: 5 / 2 = {}", 5 / 2);
    println!("Mod: 5 % 2 = {}", 5 % 2);
}