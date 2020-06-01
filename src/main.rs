use std::io::stdin;
use roblocks::Robot;

fn main() {
    let stdin = stdin();
    let mut handle = stdin.lock();

    Robot::run(&mut handle).unwrap();
}
