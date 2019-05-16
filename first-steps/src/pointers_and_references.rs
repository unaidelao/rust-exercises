pub fn run() {
    // Primitive array
    let array1 = [1, 2, 3, 4, 5];
    let array2 = array1;

    println!("Array1 and Array2 values: {:?}", (array1, array2));

    // With non-primitives, if you assign another variable to a piece of data, the first variable
    // will no longer hold that value. You will need to use a reference (&) to point to the
    // resource.

    // Vector (which is non-primitive)
    let vector1 = vec![1, 2, 3, 4, 5];
    let vector2 = &vector1;

    println!("Vector1 and Vector2 values: {:?}", (&vector1, vector2));
}