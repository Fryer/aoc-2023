pub fn part1(input: &str) {
    let hands = parse_hands(input, false);
    let total = calculate_winnings(hands);
    println!("Total winnings: {}", total);
}

pub fn part2(input: &str) {
    let hands = parse_hands(input, true);
    let total = calculate_winnings(hands);
    println!("Total winnings: {}", total);
}

fn calculate_winnings(mut hands: Vec<Hand>) -> u32 {
    hands.sort_by_key(|hand| hand.score);
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += (i as u32 + 1) * hand.bid;
    }
    return total;
}

fn parse_hands(text: &str, with_jokers: bool) -> Vec<Hand> {
    let lines = text.lines();
    return lines.map(|line| parse_hand(line, with_jokers)).collect();
}

fn parse_hand(text: &str, with_jokers: bool) -> Hand {
    let (cards, bid_text) = text.split_once(' ').unwrap();
    let score = score_cards(cards, with_jokers);
    let bid: u32 = bid_text.parse().unwrap();
    return Hand { score, bid };
}

fn score_cards(cards: &str, with_jokers: bool) -> u32 {
    let j_card = if with_jokers { 0 } else { 10 };
    let mut buckets = [0; 14];
    let mut multiple_score = 0;
    let mut symbol_score = 0;
    let mut max_bucket = 0;
    let mut jokers = 0;
    for (i, card_ch) in cards.chars().enumerate() {
        let card = score_card(card_ch, j_card);
        let bucket = &mut buckets[card as usize];
        *bucket += 1;
        if card == 0 {
            jokers += 1;
        }
        else {
            multiple_score += score_bucket_top(*bucket);
            max_bucket = std::cmp::max(max_bucket, *bucket);
        }
        symbol_score += card << 16 - 4 * i;
    }
    for _ in 0..jokers {
        max_bucket += 1;
        multiple_score += score_bucket_top(max_bucket);
    }
    multiple_score <<= 20;
    return multiple_score + symbol_score;
}

fn score_card(card_ch: char, j_card: u32) -> u32 {
    return match card_ch {
        digit if card_ch.is_ascii_digit() => digit.to_digit(10).unwrap() - 1,
        'T' => 9,
        'J' => j_card,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!(),
    };
}

fn score_bucket_top(bucket: u32) -> u32 {
    return match bucket {
        1 => 0,
        2 => 1,
        3 => 2,
        4 => 2,
        5 => 1,
        _ => panic!(),
    };
}

struct Hand {
    score: u32,
    bid: u32,
}
