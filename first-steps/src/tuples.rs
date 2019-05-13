// A tuple is a collection of values of different types.
// Tuples are constructed using parentheses.
// Max 12 elements.
// Functions can use tuples to return multiple values, as tuples can hold any number of values.

pub fn run() {
    // When declaring a tuple, we must define its elements types.
    let car: (&str, &str, u16) = ("Ford", "Pontiac", 40000);

    println!("This car is a {} {}, and its price is {}.", car.0, car.1, car.2);
}