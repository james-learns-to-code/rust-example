pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);
    println!("{:?}", numbers[4]);

    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);
}