use std::iter::once;
use itertools::repeat_n;
use ndarray::Array;

pub fn part1(input: &str) -> String {
    let conditions = parse_conditions(input);
    let mut arrangements = 0;
    for Condition { pattern, lengths } in conditions {
        arrangements += count_arrangements(pattern, &lengths);
    }
    return format!("Sum of arrangements: {}", arrangements);
}

pub fn part2(input: &str) -> String {
    let conditions = parse_conditions(input);
    let mut arrangements = 0;
    for Condition { pattern, lengths } in conditions {
        let pattern = &[pattern].repeat(5).join("?");
        let lengths = lengths.repeat(5);
        arrangements += count_arrangements(pattern, &lengths);
    }
    return format!("Sum of arrangements: {}", arrangements);
}

fn count_arrangements(pattern: &str, lengths: &[usize]) -> usize {
    let mut flags = vec!('.');
    for &length in lengths {
        flags.extend(repeat_n('#', length));
        flags.push('.');
    }
    let mut dp = Array::from_elem((pattern.len() + 3, flags.len() + 1), 0);
    dp[(0, 0)] = 1;
    for (i, ch) in once('?').chain(pattern.chars()).chain(once('?')).enumerate() {
        for (j, flag) in flags.iter().copied().enumerate() {
            dp[(i + 1, j + 1)] = match (ch, flag) {
                ('.', '#') | ('#', '.') => 0,
                ('#', '#') | ('?', '#') => dp[(i, j)],
                ('.', '.') | ('?', '.') => dp[(i, j + 1)] + dp[(i, j)],
                _ => panic!(),
            };
        }
    }
    return *dp.last().unwrap();
}

fn parse_conditions(text: &str) -> Vec<Condition> {
    let mut conditions = vec!();
    for line in text.lines() {
        let (pattern, lengths_text) = line.split_once(' ').unwrap();
        let length_texts = lengths_text.split(',');
        let lengths = length_texts.map(|text| text.parse::<usize>().unwrap()).collect();
        conditions.push(Condition { pattern, lengths });
    }
    return conditions;
}

struct Condition<'a> {
    pattern: &'a str,
    lengths: Vec<usize>,
}
