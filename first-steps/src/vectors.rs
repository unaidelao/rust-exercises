// A vector is a collection type provided by the standard library that is allowed to grow
// or shrink in size.
// 
// Vectors store more than one value in a single data structure that puts all the values next to
// each other in memory. Vectors can only store values of the same type.

use std::mem;

pub fn run() {

    // Vector declaration
    let mut names_mut: Vec<&str> = vec!["Charles", "Christian", "Klaus"];

    // We can re-assign vector values, if we use mut at vector declaration
    names_mut[1] = "Helen";

    // Push a new element at the end
    names_mut.push("Zoe");
    names_mut.push("Ingrid");

    // Pop vector's last value
    names_mut.pop();

    println!("names_mut vector values: {:?}", names_mut);

    // Vector length
    println!("names_mut vector length: {}", names_mut.len());

    // As vectors are allocated on the stack, we can get the memory size it occupies
    println!("names_mut occupies {} bytes on the stack.", mem::size_of_val(&names_mut));

    // Vector slice
    let names_mut_slice: &[&str] = &names_mut[1..3];
    println!("names_mut_slice: {:?}", names_mut_slice);

    // Loop through names_mut vector
    for x in names_mut.iter() {
        println!("Name: {}", x);
    }

    // Loop through names_mut vector while mutates values
    for x in names_mut.iter_mut() {
        *x = "mutated";
    }

    println!("Mutated vector through Loop: {:?}", names_mut);
}