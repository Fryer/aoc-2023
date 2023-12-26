use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let mut moves = vec!();
    for move_text in input.split('\n') {
        let (direction, length_text, _) = move_text.split(' ').collect_tuple().unwrap();
        let length = length_text.parse::<i64>().unwrap();
        moves.push((direction, length));
    }
    let volume = calculate_volume(&moves);
    return format!("Cubic meters: {}", volume);
}

pub fn part2(input: &str) -> String {
    let mut moves = vec!();
    for move_text in input.split('\n') {
        let (_, _, code) = move_text.split(' ').collect_tuple().unwrap();
        let length = i64::from_str_radix(&code[2..7], 16).unwrap();
        let direction = match &code[7..8] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!(),
        };
        moves.push((direction, length));
    }
    let volume = calculate_volume(&moves);
    return format!("Cubic meters: {}", volume);
}

fn calculate_volume(moves: &Vec<(&str, i64)>) -> i64 {
    let mut edge = 0;
    let mut volume = 0;
    let mut c = 0;
    let mut last_direction = moves.last().unwrap().0;
    for &(direction, length) in moves {
        edge += length;
        volume += match (last_direction, direction) {
            ("L", "U") => -c * length,
            ("R", "U") => -c * (length - 1),
            ("L", "D") => (c + 1) * (length - 1),
            ("R", "D") => (c + 1) * length,
            ("U", "L") => 0,
            ("D", "L") => c + 1,
            ("U", "R") => -c,
            ("D", "R") => 0,
            _ => panic!(),
        };
        c += match direction {
            "L" => -length,
            "R" => length,
            "U" | "D" => 0,
            _ => panic!(),
        };
        last_direction = direction;
    }
    if volume < 0 {
        return edge - volume;
    }
    return volume;
}
