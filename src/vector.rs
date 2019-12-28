pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);

    numbers.push(6);

    println!("{:?}", numbers);

    numbers.pop();

    println!("{:?}", numbers);
}