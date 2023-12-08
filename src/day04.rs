use std::collections::{HashSet, VecDeque};

pub fn part1(input: &str) -> String {
    let cards = parse_cards(input);
    let mut sum = 0;
    for card in cards {
        let mut score = 0;
        let mut next_score = 1;
        for number in card.numbers {
            if card.winnings.contains(&number) {
                score = next_score;
                next_score *= 2;
            }
        }
        sum += score;
    }
    return format!("Sum of scores: {}", sum);
}

pub fn part2(input: &str) -> String {
    let cards = parse_cards(input);
    let mut copy_queue = VecDeque::from([0]);
    let mut count = 0;
    for card in cards {
        let copies = copy_queue.pop_front().unwrap() + 1;
        count += copies;
        let mut score = 0;
        for number in card.numbers {
            if card.winnings.contains(&number) {
                score += 1;
            }
        }
        for i in 0..score {
            if i >= copy_queue.len() {
                copy_queue.push_back(copies);
                continue;
            }
            copy_queue[i] += copies;
        }
        if copy_queue.is_empty() {
            copy_queue.push_back(0);
        }
    }
    return format!("Total number of cards: {}", count);
}

fn parse_cards(text: &str) -> impl Iterator<Item = Card> + '_ {
    return text.lines().map(|line| {
        let card_text = line.split_once(": ").unwrap().1;
        let (winnings_text, numbers_text) = card_text.split_once(" | ").unwrap();
        let winnings = parse_numbers(winnings_text);
        let numbers = parse_numbers(numbers_text);
        Card::new(winnings.collect(), numbers.collect())
    });
}

fn parse_numbers(text: &str) -> impl Iterator<Item = u32> + '_ {
    return text.split(' ').filter(|x| !x.is_empty()).map(|x| x.parse().unwrap());
}

struct Card {
    winnings: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(winnings: HashSet<u32>, numbers: Vec<u32>) -> Card {
        return Card { winnings, numbers };
    }
}
