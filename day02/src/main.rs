use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = io::stdin();
    let lines: Vec<String> = input.lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let target_frequencies = [2, 3];
    let mut target_count = HashMap::<i32, i32>::new();
    for target_frequency in &target_frequencies {
        target_count.insert(*target_frequency, 0);
    }

    for line in &lines {
        let mut char_frequencies = HashMap::<char, i32>::new();
        for character in line.chars() {
            let current = char_frequencies.entry(character).or_insert_with(|| 0);
            *current += 1;
        }

        // deduplicate
        let frequencies: HashSet<_> = char_frequencies.iter()
            .map(|(_, i)| i)
            .collect();

        for frequency in frequencies {
            if let Some(count) = target_count.get_mut(&frequency) {
                *count += 1;
            }
        }
    }

    let checksum: i32 = target_count.iter().map(|(_, count)| {
        count
    }).product();

    println!("{}", checksum);
}
