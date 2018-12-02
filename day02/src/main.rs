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

    let checksum = checksum(&lines);
    println!("checksum: {}", checksum);

    let close_match = find_close_match(&lines).unwrap();
    println!("close match: {:?}", close_match);

    let (first, second) = close_match;
    let same_chars: String = first.chars()
        .zip(second.chars())
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a)
        .collect();

    println!("common characters: {}", same_chars);
}

fn checksum(lines: &Vec<String>) -> i32 {
    let target_frequencies = [2, 3];
    let mut target_count = HashMap::<i32, i32>::new();
    for target_frequency in &target_frequencies {
        target_count.insert(*target_frequency, 0);
    }

    for line in lines {
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

    target_count.iter().map(|(_, count)| {
        count
    }).product()
}

fn find_close_match(lines: &Vec<String>) -> Option<(&String, &String)> {
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if differ_by_exactly_one(&lines[i], &lines[j]) {
                return Some((&lines[i], &lines[j]));
            }
        }
    }
    return None;
}

fn differ_by_exactly_one(lhs: &String, rhs: &String) -> bool {
    let mut diff_count = 0;
    for (a, b) in lhs.chars().zip(rhs.chars()) {
        if a != b {
            if diff_count > 0 {
                return false;
            }
            diff_count += 1;
        }
    };

    true
}
