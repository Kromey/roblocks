use std::io::stdin;
use roblocks::Command;

fn main() {
    let stdin = stdin();
    let mut handle = stdin.lock();

    Command::read(&mut handle);
}
