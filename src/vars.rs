pub fn run() {
    let mut name = "Bard";
    name = "Brad";
    println!("{}", name);

    const ID: i32 = 001;
    println!("{}", ID);

    let ( name, age ) = ("James", 30);
    println!("{} {}", name, age)
}