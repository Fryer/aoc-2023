use std::{fs, io};
use std::io::Write;
use crate::days::Part;

mod days;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

const DAYS: &[days::Day] = &[
    &[
        Part::new("Part 1", day01::part1),
        Part::new("Part 2", day01::part2),
    ],
    &[
        Part::new("Part 1", day02::part1),
        Part::new("Part 2", day02::part2),
    ],
    &[
        Part::new("Part 1", day03::part1),
        Part::new("Part 2", day03::part2),
    ],
    &[
        Part::new("Part 1", day04::part1),
        Part::new("Part 2", day04::part2),
    ],
    &[
        Part::new("Part 1", day05::part1),
        Part::new("Part 2", day05::part2),
    ],
];

fn main() {
    print!("Day: ");
    io::stdout().flush().unwrap();
    let mut io_input = String::new();
    io::stdin().read_line(&mut io_input).unwrap();
    let day: usize = match io_input.trim_end().parse() {
        Ok(day) => day,
        Err(_) => {
            println!("Day must be 1-25.");
            return;
        }
    };
    if day < 1 || day > 25 {
        println!("Day must be 1-25.");
        return;
    }
    if day > DAYS.len() {
        println!("Day {} not implemented.", day);
        return;
    }
    let input = match fs::read_to_string(format!("input/day{:02}.txt", day)) {
        Ok(data) => data.replace('\r', ""),
        Err(_) => {
            println!("Input file \"input/day{:02}.txt\" not found.", day);
            return;
        }
    };
    for part in DAYS[day - 1] {
        println!("=== {} ===", part.name);
        let start = std::time::Instant::now();
        (part.run)(&input);
        println!("Time: {} µs", start.elapsed().as_micros());
    }
}
