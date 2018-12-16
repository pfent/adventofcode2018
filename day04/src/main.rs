extern crate regex;

use std::io;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

struct Event {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    event: String,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}-{}-{} {}:{}] {}", self.year, self.month, self.day, self.hour, self.min, self.event)
    }
}


fn main() {
    let regex = Regex::new(r"\[(\d*)-(\d*)-(\d*) (\d*):(\d*)] (.*)").unwrap();

    let input = io::stdin();
    let mut input: Vec<Event> = input.lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let capture = regex.captures(&line).unwrap();
            Event {
                year: capture.get(1).unwrap().as_str().parse().unwrap(),
                month: capture.get(2).unwrap().as_str().parse().unwrap(),
                day: capture.get(3).unwrap().as_str().parse().unwrap(),
                hour: capture.get(4).unwrap().as_str().parse().unwrap(),
                min: capture.get(5).unwrap().as_str().parse().unwrap(),
                event: capture.get(6).unwrap().as_str().parse().unwrap(),
            }
        }).collect();

    input.sort_by(|a, b|
        if a.year != b.year {
            a.year.cmp(&b.year)
        } else if a.month != b.month {
            a.month.cmp(&b.month)
        } else if a.day != b.day {
            a.day.cmp(&b.day)
        } else if a.hour != b.hour {
            a.hour.cmp(&b.hour)
        } else {
            a.min.cmp(&b.min)
        }
    );

    // guard_no -> sleep_times
    let mut guard_tracker = HashMap::<i32, Vec<(i32, i32)>>::new();

    let guard_regex = Regex::new(r"Guard #(\d*) begins shift").unwrap();
    let asleep_regex = Regex::new(r"falls asleep").unwrap();
    let wakeup_regex = Regex::new(r"wakes up").unwrap();

    let mut current_guard = -1;
    let mut begin_sleep_min = -1;
    for i in input {
        if let Some(capture) = guard_regex.captures(&i.event) {
            current_guard = capture.get(1).unwrap().as_str().parse().unwrap();
        } else if asleep_regex.is_match(&i.event) {
            begin_sleep_min = i.min;
        } else if wakeup_regex.is_match(&i.event) {
            let end_sleep_min = i.min;
            let entry = guard_tracker.entry(current_guard).or_default();
            entry.push((begin_sleep_min, end_sleep_min));
        } else {
            panic!("unknown event");
        }
    }

    let guard_total_sleep: Vec<(i32, i32)> = guard_tracker.iter().map(|(guard, times)| {
        let sleep_time: i32 = times.iter().map(|(begin, end)| {
            end - begin
        }).sum();
        return (*guard, sleep_time);
    }).collect();

    let (sleepy_guard, total_time) = guard_total_sleep.iter().max_by(|(_, a), (_, b)| {
        return a.cmp(b);
    }).unwrap();

    println!("{} sleeps max time {}", sleepy_guard, total_time);

    // minute -> sleep count
    let mut sleep_histogramm = HashMap::<i32, i32>::new();

    guard_tracker[sleepy_guard].iter().for_each(|(sleep_begin, sleep_end)| {
        for i in *sleep_begin..*sleep_end {
            let entry = sleep_histogramm.entry(i).or_default();
            *entry += 1;
        }
    });

    let (sleepy_minute, sleepy_count) = sleep_histogramm.iter().max_by(|(_, count_a), (_, count_b)| {
        return count_a.cmp(count_b);
    }).unwrap();

    println!("{} was the most sleepy minute with {} times asleep", sleepy_minute, sleepy_count);

    println!("{} * {} = {}", sleepy_guard, sleepy_minute, sleepy_guard * sleepy_minute);
}
