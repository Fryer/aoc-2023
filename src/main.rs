use std::{fs, io};
use std::io::Write;
use std::time::Instant;
use crate::days::Part;

mod days;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

const DAYS: &[days::Day] = &[
    &[
        Part::new("Day 1, part 1", day01::part1),
        Part::new("Day 1, part 2", day01::part2),
    ],
    &[
        Part::new("Day 2, part 1", day02::part1),
        Part::new("Day 2, part 2", day02::part2),
    ],
    &[
        Part::new("Day 3, part 1", day03::part1),
        Part::new("Day 3, part 2", day03::part2),
    ],
    &[
        Part::new("Day 4, part 1", day04::part1),
        Part::new("Day 4, part 2", day04::part2),
    ],
    &[
        Part::new("Day 5, part 1", day05::part1),
        Part::new("Day 5, part 2", day05::part2),
    ],
    &[
        Part::new("Day 6, part 1", day06::part1),
        Part::new("Day 6, part 2", day06::part2),
    ],
];

fn main() {
    print!("Day (default = all): ");
    io::stdout().flush().unwrap();
    let day_input = io::stdin().lines().next().unwrap().unwrap();
    let timer = Instant::now();
    if day_input.is_empty() {
        for day in 1..=DAYS.len() {
            match read_day_input(day) {
                Ok(input) => run_day(day, &input),
                Err(_) => return,
            };
        }
    }
    else {
        let day: usize = match parse_day(&day_input) {
            Ok(day) => day,
            Err(_) => return,
        };
        match read_day_input(day) {
            Ok(input) => run_day(day, &input),
            Err(_) => return,
        };
    }
    println!("=== Done! ===");
    println!("Total time: {} µs", timer.elapsed().as_micros());
}

fn parse_day(text: &str) -> Result<usize, ()> {
    let day: usize = match text.trim_end().parse() {
        Ok(day) => day,
        Err(_) => {
            println!("Day must be 1-25.");
            return Err(());
        },
    };
    if day < 1 || day > 25 {
        println!("Day must be 1-25.");
        return Err(());
    }
    if day > DAYS.len() {
        println!("Day {} not implemented.", day);
        return Err(());
    }
    return Ok(day);
}

fn read_day_input(day: usize) -> Result<String, ()> {
    return match fs::read_to_string(format!("input/day{:02}.txt", day)) {
        Ok(data) => Ok(data.replace('\r', "")),
        Err(_) => {
            println!("Input file \"input/day{:02}.txt\" not found.", day);
            return Err(());
        },
    };
}

fn run_day(day: usize, input: &str) {
    for part in DAYS[day - 1] {
        println!("=== {} ===", part.name);
        let timer = Instant::now();
        (part.run)(input);
        println!("Time: {} µs", timer.elapsed().as_micros());
    }
}
