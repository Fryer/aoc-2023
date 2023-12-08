pub fn part1(input: &str) -> String {
    const MAX_SET: Set = Set::new(12, 13, 14);

    let mut sum = 0;
    let lines = input.lines();
    let games = lines.map(parse_game);
    for game in games {
        let mut valid = true;
        for set in game.sets {
            if set.red > MAX_SET.red || set.green > MAX_SET.green || set.blue > MAX_SET.blue {
                valid = false;
                break;
            }
        }
        if valid {
            sum += game.id;
        }
    }
    return format!("Sum of game IDs: {}", sum);
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;
    let lines = input.lines();
    let games = lines.map(parse_game);
    for game in games {
        let mut min_set = Set::new(0, 0, 0);
        for set in game.sets {
            min_set.red = std::cmp::max(min_set.red, set.red);
            min_set.green = std::cmp::max(min_set.green, set.green);
            min_set.blue = std::cmp::max(min_set.blue, set.blue);
        }
        let power = min_set.red * min_set.green * min_set.blue;
        sum += power;
    }
    return format!("Sum of games' minimum set powers: {}", sum);
}

fn parse_game(text: &str) -> Game<impl IntoIterator<Item = Set> + '_> {
    let colon_index = text.find(':').unwrap();
    let id = text[5..colon_index].parse().unwrap();
    let sets = text[colon_index + 1..].split(';').map(parse_set);
    return Game::new(id, sets);
}

fn parse_set(text: &str) -> Set {
    let [mut red, mut green, mut blue] = [0, 0, 0];
    let colors = text.split(',').map(|x| x.trim_start());
    for color in colors {
        let (amount, name) = color.split_once(' ').unwrap();
        let amount: u32 = amount.parse().unwrap();
        match name {
            "red" => red += amount,
            "green" => green += amount,
            "blue" => blue += amount,
            _ => panic!(),
        }
    }
    return Set::new(red, green, blue);
}

struct Game<T: IntoIterator<Item = Set>> {
    id: u32,
    sets: T,
}

impl<T: IntoIterator<Item = Set>> Game<T> {
    const fn new(id: u32, sets: T) -> Game<T> {
        return Game { id, sets };
    }
}

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    const fn new(red: u32, green: u32, blue: u32) -> Set {
        return Set { red, green, blue };
    }
}