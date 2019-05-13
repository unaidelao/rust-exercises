// Rust is a statically typed language, however, the compiler can usually infer what type we want
// to use based on the value and how we use it.

// u16 means unsigned 16 bit integer; i32 means signed 32 bit integer.
pub fn run() {
    // Default integer type is i32.
    let num1 = 2;

    // Default float is f64.
    let num2 = 4.2;

    // We can asign an explicit data type.
    let num3: i8 = -30;

    // Max size
    println!("Max u8: {}.", std::u8::MAX);
    println!("Max i8: {}.", std::i8::MAX);
    println!("Max f32: {}.", std::f32::MAX);
    println!("Max f64: {}.", std::f64::MAX);
    println!("Max i128: {}.", std::i128::MAX);
    println!("Max u128: {}.", std::u128::MAX);

    // Boolean variables. 
    let flag1 = true;
    let flag2: bool = false;
    let is_greater = 50 < 25;

    // Characters. Needed to use single quotes (like Java).
    let char1 = 'v';
    let book_unicode_emoji = '\u{1F4DA}';

    println!("Values: {:?}", (num1, num2, num3, flag1, flag2, is_greater, book_unicode_emoji));
}