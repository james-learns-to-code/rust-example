
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
}

pub fn run() {
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("{}", color.red);

    color.red = 200;

    println!("{} {} {}", color.red, color.green, color.blue);


    let mut person = Person::new("John","Doe");

    println!("{}", person.full_name());

    person.set_last_name("Lee");

    println!("{}", person.full_name());
}