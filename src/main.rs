// mod day2;
// mod day3;
mod day4;
use std::io::{self, Read};

fn main() {
    // let contents = fs::read_to_string("./input/input3.txt").unwrap();
    let mut contents = String::new();
    let _ = io::stdin()
        .lock()
        .read_to_string(&mut contents)
        .expect("failed to read input");

    println!("{}", contents);

    println!("Solution: {}", day4::solve(&contents));
}
