enum Movement { Up, Down, Left, Right }

fn move_car(m: Movement) {
    match m {
        Movement::Up => println!("Car moves up."),
        Movement::Down => println!("Car moves down."),
        Movement::Left => println!("Car moves left."),
        Movement::Right => println!("Car moves right.")
    }
}

pub fn run() {
    let car1 = Movement::Down;
    let car2 = Movement::Left;
    let car3 = Movement::Right;
    let car4 = Movement::Up;

    move_car(car1);
    move_car(car2);
    move_car(car3);
    move_car(car4);
}