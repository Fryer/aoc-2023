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
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod canvas;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

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
    &[
        Part::new("Day 7, part 1", day07::part1),
        Part::new("Day 7, part 2", day07::part2),
    ],
    &[
        Part::new("Day 8, part 1", day08::part1),
        Part::new("Day 8, part 2", day08::part2),
    ],
    &[
        Part::new("Day 9, part 1", day09::part1),
        Part::new("Day 9, part 2", day09::part2),
    ],
    &[
        Part::new("Day 10, part 1", day10::part1),
        Part::new("Day 10, part 2", day10::part2),
    ],
    &[
        Part::new("Day 11, part 1", day11::part1),
        Part::new("Day 11, part 2", day11::part2),
    ],
    &[
        Part::new("Day 12, part 1", day12::part1),
        Part::new("Day 12, part 2", day12::part2),
    ],
    &[
        Part::new("Day 13, part 1", day13::part1),
        Part::new("Day 13, part 2", day13::part2),
    ],
    &[
        Part::new("Day 14, part 1", day14::part1),
        Part::new("Day 14, part 2", day14::part2),
    ],
    &[
        Part::new("Day 15, part 1", day15::part1),
        Part::new("Day 15, part 2", day15::part2),
    ],
    &[
        Part::new("Day 16, part 1", day16::part1),
        Part::new("Day 16, part 2", day16::part2),
        Part::new("Day 16 visualization", day16::visualize),
    ],
    &[
        Part::new("Day 17, part 1", day17::part1),
        Part::new("Day 17, part 2", day17::part2),
    ],
    &[
        Part::new("Day 18, part 1", day18::part1),
        Part::new("Day 18, part 2", day18::part2),
    ],
    &[
        Part::new("Day 19, part 1", day19::part1),
        Part::new("Day 19, part 2", day19::part2),
    ],
    &[
        Part::new("Day 20, part 1", day20::part1),
        Part::new("Day 20, part 2", day20::part2),
    ],
    &[
        Part::new("Day 21, part 1", day21::part1),
        Part::new("Day 21, part 2", day21::part2),
    ],
    &[
        Part::new("Day 22, part 1", day22::part1),
        Part::new("Day 22, part 2", day22::part2),
    ],
    &[
        Part::new("Day 23, part 1", day23::part1),
        Part::new("Day 23, part 2", day23::part2),
    ],
    &[
        Part::new("Day 24, part 1", day24::part1),
        Part::new("Day 24, part 2", day24::part2),
    ],
];

fn main() {
    print!("Day (default = all, add '+' for extras): ");
    io::stdout().flush().unwrap();
    let day_input = io::stdin().lines().next().unwrap().unwrap();
    let mut output = String::new();
    let timer = Instant::now();
    if day_input.is_empty() {
        for day in 1..=DAYS.len() {
            match read_day_input(day) {
                Ok(input) => output += &run_day(day, &input, false),
                Err(_) => return,
            };
        }
    }
    else {
        let (day, extras) = match parse_day(&day_input) {
            Ok((day, extra)) => (day, extra),
            Err(_) => return,
        };
        if extras && DAYS[day - 1].len() < 3 {
            println!("Day {} extras not added.", day);
            return;
        }
        match read_day_input(day) {
            Ok(input) => output += &run_day(day, &input, extras),
            Err(_) => return,
        };
    }
    output += "=== Done! ===\n";
    output += &format!("Total time: {} µs\n", timer.elapsed().as_micros());
    print!("{}", output);
}

fn parse_day(text: &str) -> Result<(usize, bool), ()> {
    let extras = text.find('+').is_some();
    let day_text = if extras { &text[0..text.len() - 1] } else { text };
    let day: usize = match day_text.trim_end().parse() {
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
    return Ok((day, extras));
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

fn run_day(day: usize, input: &str, extras: bool) -> String {
    let mut output = String::new();
    let parts = DAYS[day - 1];
    let parts = if extras { &parts[2..] } else { &parts[0..2.min(parts.len())] };
    for part in parts {
        output += &format!("=== {} ===\n", part.name);
        let timer = Instant::now();
        output += &((part.run)(input) + "\n");
        output += &format!("Time: {} µs\n", timer.elapsed().as_micros());
    }
    return output;
}
