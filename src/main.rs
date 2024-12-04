mod day3;
use std::io::{self, Read};

fn main() {
    // let contents = fs::read_to_string("./input/input3.txt").unwrap();
    let mut contents = String::new();
    let _ = io::stdin()
        .lock()
        .read_to_string(&mut contents)
        .expect("failed to read input");

    println!("{}", contents);

    println!("Solution: {}", day3::solve(&contents));
}
