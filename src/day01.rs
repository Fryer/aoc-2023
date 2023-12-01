pub fn part1(input: &str) {
    let mut sum = 0;
    let lines = input.lines();
    for line in lines {
        let digits: Vec<&str> = line.matches(char::is_numeric).collect();
        let first: u32 = digits.first().unwrap().parse().unwrap();
        let last: u32 = digits.last().unwrap().parse().unwrap();
        let calibration = 10 * first + last;
        sum += calibration;
    }
    println!("Sum of calibration values: {}", sum);
}

pub fn part2(input: &str) {
    const DIGIT_NAMES: &[(u32, &str)] = &[
        (1, "one"), (1, "1"),
        (2, "two"), (2, "2"),
        (3, "three"), (3, "3"),
        (4, "four"), (4, "4"),
        (5, "five"), (5, "5"),
        (6, "six"), (6, "6"),
        (7, "seven"), (7, "7"),
        (8, "eight"), (8, "8"),
        (9, "nine"), (9, "9"),
    ];

    let mut sum = 0;
    let lines = input.lines();
    for line in lines {
        let mut first: Option<u32> = None;
        let mut first_index: Option<usize> = None;
        let mut last: Option<u32> = None;
        let mut last_index: Option<usize> = None;
        for (digit, name) in DIGIT_NAMES {
            let index = line.find(name);
            if index.is_some() && index.unwrap() <= first_index.unwrap_or(usize::MAX) {
                first = Some(*digit);
                first_index = index;
            }
            let index = line.rfind(name);
            if index.is_some() && index.unwrap() >= last_index.unwrap_or(0) {
                last = Some(*digit);
                last_index = index;
            }
        }
        let calibration = 10 * first.unwrap() + last.unwrap();
        sum += calibration;
    }
    println!("Sum of calibration values: {}", sum);
}
