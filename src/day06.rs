pub fn part1(input: &str) {
    let races = parse_races(input, parse_numbers_separate);
    let combinations: u64 = races.map(|race| calculate_wins(race)).product();
    println!("Number of ways to win: {}", combinations);
}

pub fn part2(input: &str) {
    let race = parse_races(input, parse_numbers_together).next().unwrap();
    let wins = calculate_wins(race);
    println!("Number of ways to win: {}", wins);
}

fn calculate_wins(race: Race) -> u64 {
    let t = race.time as f64;
    let d = race.distance as f64;
    let sqrt = (t * t - 4. * d).sqrt();
    let x1 = (t - sqrt) / 2.;
    let x2 = (t + sqrt) / 2.;
    let shortest = if x1.fract() == 0. { x1 + 1. } else { x1.ceil() } as u64;
    let longest = if x2.fract() == 0. { x2 - 1. } else { x2.floor() } as u64;
    return longest - shortest + 1;
}

fn parse_races<'a, F, I>(text: &'a str, parse_numbers: F) -> impl Iterator<Item = Race> + 'a
where
    F: Fn(&'a str) -> I,
    I: Iterator<Item = u64> + 'a
{
    let (time_texts, distance_texts) = text.split_once('\n').unwrap();
    let times = parse_numbers(time_texts);
    let distances = parse_numbers(distance_texts);
    return times.zip(distances).map(|(time, distance)| Race { time, distance });
}

fn parse_numbers_separate<'a>(text: &'a str) -> impl Iterator<Item = u64> + 'a {
    let numbers_text = text.split_once(':').unwrap().1.trim_start();
    let number_texts = numbers_text.split_whitespace();
    return number_texts.map(|x| x.parse().unwrap());
}

fn parse_numbers_together<'a>(text: &'a str) -> impl Iterator<Item = u64> + 'a {
    let numbers_text = text.split_once(':').unwrap().1;
    let number_text = numbers_text.replace(' ', "");
    return std::iter::once(number_text.parse().unwrap());
}

struct Race {
    time: u64,
    distance: u64,
}
