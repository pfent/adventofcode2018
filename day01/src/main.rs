use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();
    let sum: i32 = input.lock()
        .lines()
        .map(&|line: Result<String, std::io::Error>| {
            line.unwrap()
                .parse::<i32>()
                .unwrap()
        }).sum();

    println!("{}", sum);
}
