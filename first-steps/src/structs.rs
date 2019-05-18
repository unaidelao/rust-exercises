// They are like C structs.
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct ColorTuple(u8, u8, u8);

// More elaborated struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Return full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Setter last_name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // first_name and last_name to Tuple
    fn full_name_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // Traditional struct example
    let mut color1 = Color { red: 0, green: 255, blue: 0 };

    color1.red = 70;
    color1.blue = 220;

    println!("Color1: {}, {}, {}.", color1.red, color1.green, color1.blue);

    // Tuple struct example
    let mut color2 = ColorTuple(150, 50, 0);

    color2.2 = 10;

    println!("Color2: {}, {}, {}.", color2.0, color2.1, color2.2);

    // More elaborated struct example
    let mut person1 = Person::new("Perico", "Palotes");
    println!("Person1: {} {}", person1.first_name, person1.last_name);

    println!("{}", person1.full_name());

    person1.set_last_name("Romero");

    println!("{}", person1.full_name());

    println!("Person1 Tuple: {:?}", person1.full_name_tuple());
}