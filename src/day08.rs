use std::collections::HashMap;
use num_integer::Integer;

pub fn part1(input: &str) -> String {
    let NodeMap { path, node_list } = parse_map(input, false);
    let NodeList { nodes, firsts, lasts } = node_list;
    let first = firsts[0];
    let last = lasts[0];
    let mut node = first;
    let mut steps = 0;
    for &step in path.iter().cycle() {
        node = nodes[node][step];
        steps += 1;
        if node == last {
            break;
        }
    }
    return format!("Steps to reach end: {}", steps);
}

pub fn part2(input: &str) -> String {
    let NodeMap { path, node_list } = parse_map(input, true);
    let NodeList { nodes, firsts, lasts } = node_list;
    let mut last_flags = Vec::with_capacity(nodes.len());
    last_flags.resize(nodes.len(), false);
    for last in lasts {
        last_flags[last] = true;
    }
    let mut cycles = Vec::with_capacity(firsts.len());
    for first in firsts {
        let mut node = first;
        let mut cycle: usize = 0;
        for &step in path.iter().cycle() {
            node = nodes[node][step];
            cycle += 1;
            if last_flags[node] {
                cycles.push(cycle);
                break;
            }
        }
    }
    let gcd = cycles.iter().copied().reduce(|gcd, cycle| gcd.gcd(&cycle)).unwrap();
    let steps = cycles.iter().copied().reduce(|steps, cycle| steps * cycle / gcd).unwrap();
    return format!("Steps to reach ends simultaneously: {}", steps);
}

fn parse_map(text: &str, with_ghosts: bool) -> NodeMap {
    let (path_text, nodes_text) = text.split_once("\n\n").unwrap();
    let path = parse_path(path_text);
    let node_list = parse_nodes(nodes_text, with_ghosts);
    return NodeMap { path, node_list };
}

fn parse_path(text: &str) -> Vec<usize> {
    return text.chars().map(|ch| if ch == 'L' { 0 } else { 1 }).collect();
}

fn parse_nodes(text: &str, with_ghosts: bool) -> NodeList {
    let mut mapping = HashMap::new();
    let mut firsts = vec!();
    let mut lasts = vec!();
    for (i, line) in text.lines().enumerate() {
        let key = &line[0..3];
        mapping.insert(key, i);
        if with_ghosts && &key[2..3] == "A" {
            firsts.push(i);
        }
        if with_ghosts && &key[2..3] == "Z" {
            lasts.push(i);
        }
    }
    let mut nodes = Vec::with_capacity(mapping.len());
    for line in text.lines() {
        let left = mapping[&line[7..10]];
        let right = mapping[&line[12..15]];
        nodes.push([left, right]);
    }
    if !with_ghosts {
        firsts.push(mapping["AAA"]);
        lasts.push(mapping["ZZZ"]);
    }
    return NodeList { nodes, firsts, lasts };
}

struct NodeMap {
    path: Vec<usize>,
    node_list: NodeList,
}

struct NodeList {
    nodes: Vec<[usize; 2]>,
    firsts: Vec<usize>,
    lasts: Vec<usize>,
}
