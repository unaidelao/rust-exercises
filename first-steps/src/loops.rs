pub fn run() {
    let mut counter = 0;

    // Infinite loop
    // loop {
    //     counter += 1;
    //     println!("#{}", counter);

    //     if counter == 15 {
    //         break;
    //     }
    // }

    // While loop (FizzBuzz)
    // while counter <= 100 {
    //     if counter % 15 == 0 {
    //         println!("fizz-buzz");
    //     } else if counter % 3 == 0 {
    //         println!("fizz");
    //     } else if counter % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", counter);
    //     }
    //     counter += 1;
    // }

    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizz-buzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }

    
}