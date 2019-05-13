// Primitive str = Inmutable fixed-length string allocated somewhere in memory.
// String = Growable, heap-allocated data structure.
pub fn run() {
    let primitive_str = "Rust";
    let mut growable_str = String::from("Language");

    println!("{0} {1}", primitive_str, growable_str);

    println!("Length of primitive_str: {}.", primitive_str.len());
    println!("Length of growable_str: {}.", growable_str.len());

    // Push only a character.
    growable_str.push('!');

    // Push various characters (a string).
    growable_str.push_str("is fun!");

    println!("{0} {1}", primitive_str, growable_str);

    // Capacity (only for String) in bytes.
    println!("growable_str capacity = {}", growable_str.capacity());

    // Check if contains something expected.
    println!("Does growable_str contains \"fun\"?: {}", growable_str.contains("fun"));

    // Replace function.
    println!("Replace function: {}", growable_str.replace("Language", "Programming"));

    // For loop through String by whitespace.
    for word in growable_str.split_whitespace() {
        println!("{}", word);
    }

    // Create String with some capacity.
    let mut string_variable = String::with_capacity(20);
    string_variable.push('U');
    string_variable.push('N');
    string_variable.push('A');
    string_variable.push('I');

    println!("{}", string_variable);

    // Assert equals.
    assert_eq!(4, string_variable.len());
    assert_eq!(20, string_variable.capacity());

    assert_eq!(4, string_variable.len(), "Testing variable length");

}