pub fn run() {
    let mut count = 0;

    loop {
        count += 1;
        println!("{}", count);

        if count > 20 { break; }
    }

    for x in 0..100 {
        println!("{}", x);
    }

    let numbers: [i32; 5] = [1,2,3,4,5];

    for x in numbers.iter() {
        println!("{}", x);
    }
}