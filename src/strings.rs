pub fn run() {
    let mut hello = String::from("Hello");

    hello.push_str(" World");
    hello = hello.replace("World", "There");

    println!("{}", hello);
}