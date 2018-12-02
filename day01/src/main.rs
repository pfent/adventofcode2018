use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let input = io::stdin();
    let lines = input.lock().lines();
    let numbers: Vec<i32> = lines.map(&|line: Result<String, std::io::Error>| {
        line.unwrap()
            .parse::<i32>()
            .unwrap()
    }).collect();

    let sum = sum_values(&numbers);
    println!("{}", sum);

    let duplicate: i32 = find_first_duplicate_sum(&numbers);
    println!("{}", duplicate);
}

/// For part one
fn sum_values(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

/// For part two
fn find_first_duplicate_sum(numbers: &Vec<i32>) -> i32 {
    let mut seen_numbers = HashSet::<i32>::new();
    let mut current: i32 = 0;
    loop {
        for number in numbers {
            if seen_numbers.contains(&current) {
                return current;
            }
            seen_numbers.insert(current);
            current += number;
        }
    }
}
