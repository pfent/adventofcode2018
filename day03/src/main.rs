extern crate regex;

use std::io;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashSet;

struct Area {
    claim_no: usize,
    x: usize,
    y: usize,
    x_len: usize,
    y_len: usize,
}

fn main() {
    let regex = Regex::new(r"#(\d)* @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

    let input = io::stdin();
    let input: Vec<Area> = input.lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let capture = regex.captures(&line).unwrap();
            Area {
                claim_no: capture.get(1).unwrap().as_str().parse().unwrap(),
                x: capture.get(2).unwrap().as_str().parse().unwrap(),
                y: capture.get(3).unwrap().as_str().parse().unwrap(),
                x_len: capture.get(4).unwrap().as_str().parse().unwrap(),
                y_len: capture.get(5).unwrap().as_str().parse().unwrap(),
            }
        }).collect();

    let mut fabric = [[0; 1000]; 1000];

    input.iter().for_each(|area: &Area| {
        for i in area.x..(area.x + area.x_len) {
            for j in area.y..(area.y + area.y_len) {
                fabric[i][j] += 1;
            }
        }
    });

    let overlapping = count_overlapping(&fabric);
    println!("overlapping squares: {}", overlapping);

    let claims: HashSet<usize> = input.iter()
        .map(|area: &Area| { area.claim_no })
        .collect();


    eliminate_overlapping(&fabric, claims);
}

fn count_overlapping(fabric: &[[i32; 1000]; 1000]) -> usize {
    fabric.iter()
        .flat_map(|row| row.iter())
        .filter(|i| **i > 1)
        .count()
}

fn eliminate_overlapping(fabric: &[[i32; 1000]; 1000], mut claims: HashSet<usize>) -> HashSet<usize> {
    fabric.iter()
        .flat_map(|row| row.iter())
        .filter(|i| **i > 1);
        //.for_each(|i| claims.remove(i.into()))

    claims
}


