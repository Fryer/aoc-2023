pub fn part1(input: &str) -> String {
    let series = parse_series(input);
    let predictions = series.iter().map(predict);
    let sum: i32 = predictions.sum();
    return format!("Sum of predictions: {}", sum);
}

pub fn part2(input: &str) -> String {
    let mut series = parse_series(input);
    for values in &mut series {
        values.reverse();
    }
    let predictions = series.iter().map(predict);
    let sum: i32 = predictions.sum();
    return format!("Sum of predictions: {}", sum);
}

fn predict(values: &Vec<i32>) -> i32 {
    let window = values.iter().zip(values.iter().skip(1));
    let last_value = *values.last().unwrap();
    let differences: Vec<i32> = window.map(|(a, b)| b - a).collect();
    if differences.iter().all(|&x| x == 0) {
        return last_value;
    }
    let next_difference = predict(&differences);
    return last_value + next_difference;
}

fn parse_series(text: &str) -> Vec<Vec<i32>> {
    let series_texts = text.lines();
    let series = series_texts.map(parse_values);
    return series.collect();
}

fn parse_values(text: &str) -> Vec<i32> {
    let value_texts = text.split(' ');
    let values = value_texts.map(|value| value.parse::<i32>().unwrap());
    return values.collect();
}
