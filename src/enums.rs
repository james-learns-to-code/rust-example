
enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right")
    }
}

pub fn run() {
    let up = Movement::Up;
    let down = Movement::Down;
    let left = Movement::Left;
    let right = Movement::Right;
    move_avatar(up);
    move_avatar(down);
    move_avatar(left);
    move_avatar(right);
}