// Arrays are fixed-size sequences of N elements of type T.
// Arrays are allocated on the stack, instead of heap.

use std::mem;

pub fn run() {

    let names = ["Peter", "Sofia", "James"];
    println!("names array values: {:?}", names);

    // Defining the array elements data type
    let even_array: [i32; 5] = [2, 4, 6, 8, 10];
    println!("even_array values: {:?}", even_array);

    // Accesing single values of an array
    println!("First value of names array: {}", names[0]);

    // We can re-assign array values using mut
    let mut names_mut = ["Charles", "Christian", "Klaus"];
    names_mut[1] = "Helen";

    println!("names_mut array values: {:?}", names_mut);

    // Array length
    println!("names_mut array length: {}", names_mut.len());

    // As arrays are allocated on the stack, we can get the memory size it occupies
    println!("names_mut occupies {} bytes on the stack.", mem::size_of_val(&names_mut));

    // Array slice
    let even_array_slice: &[i32] = &even_array[1..3];
    println!("even_array_slice: {:?}", even_array_slice);

}