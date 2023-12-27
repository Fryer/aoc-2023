use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let (workflows_text, parts_text) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_text);
    let mut sum = 0;
    let part_texts = parts_text.split('\n').map(|t| &t[1..t.len() - 1]);
    for text in part_texts {
        let ratings = text.split(',').map(|t| (&t[2..]).parse::<u64>().unwrap());
        let ratings = ratings.enumerate().fold([0; 4], |mut a, (i, r)| { a[i] = r; a });
        let mut location = "in";
        while location != "A" && location != "R" {
            let workflow = &workflows[location];
            location = workflow.pass;
            for &Rule { category, is_lt, number, destination } in &workflow.rules {
                let matches_lt = is_lt && ratings[category] < number;
                let matches_gt = !is_lt && ratings[category] > number;
                if matches_lt || matches_gt {
                    location = destination;
                    break;
                }
            }
        }
        if location == "A" {
            sum += ratings.iter().sum::<u64>();
        }
    }
    return format!("Sum of ratings: {}", sum);
}

pub fn part2(input: &str) -> String {
    let (workflows_text, _) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_text);
    let mut combinations = 0;
    let mut ranges_stack = vec!(([1..=4000, 1..=4000, 1..=4000, 1..=4000], "in"));
    while let Some((mut ranges, mut location)) = ranges_stack.pop() {
        if location == "R" {
            continue;
        }
        if location == "A" {
            combinations += ranges.iter().fold(1, |p, r| p * (r.end() - r.start() + 1));
            continue;
        }
        let workflow = &workflows[location];
        location = workflow.pass;
        for &Rule { category, is_lt, number, destination } in &workflow.rules {
            let matches_lt = is_lt && *ranges[category].end() < number;
            let matches_gt = !is_lt && *ranges[category].start() > number;
            if matches_lt || matches_gt {
                location = destination;
                break;
            }
            if is_lt && *ranges[category].start() < number {
                let mut branch = ranges.clone();
                branch[category] = *ranges[category].start()..=number - 1;
                ranges_stack.push((branch, destination));
                ranges[category] = number..=*ranges[category].end();
            }
            if !is_lt && *ranges[category].end() > number {
                let mut branch = ranges.clone();
                branch[category] = number + 1..=*ranges[category].end();
                ranges_stack.push((branch, destination));
                ranges[category] = *ranges[category].start()..=number;
            }
        }
        ranges_stack.push((ranges, location));
    }
    return format!("Distinct combinations: {}", combinations);
}

fn parse_workflows(text: &str) -> HashMap<&str, Workflow> {
    let mut workflows = HashMap::new();
    for text in text.split('\n') {
        let (name, rules_text) = text.split_once('{').unwrap();
        let rules_text = &rules_text[0..rules_text.len() - 1];
        let mut rule_texts = rules_text.split(',');
        let pass = rule_texts.next_back().unwrap();
        let mut rules = vec!();
        for text in rule_texts {
            let (condition, destination) = text.split_once(':').unwrap();
            let category = match &condition[0..1] {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => panic!(),
            };
            let is_lt = match &condition[1..2] {
                "<" => true,
                ">" => false,
                _ => panic!(),
            };
            let number = (&condition[2..]).parse::<u64>().unwrap();
            rules.push(Rule { category, is_lt, number, destination });
        }
        workflows.insert(name, Workflow { rules, pass });
    }
    return workflows;
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    pass: &'a str,
}

struct Rule<'a> {
    category: usize,
    is_lt: bool,
    number: u64,
    destination: &'a str,
}
