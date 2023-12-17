use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use ndarray::{Array, Array2, Axis, ShapeBuilder};

pub fn part1(input: &str) -> String {
    let mut rocks = parse_rocks(input);
    tilt_north(&mut rocks);
    let load = calculate_load(&rocks);
    return format!("Total beam load: {}", load);
}

pub fn part2(input: &str) -> String {
    let mut rocks = parse_rocks(input);
    let mut rocks_cycles = HashMap::new();
    let mut loads = vec!();
    let cycle_length: usize;
    let mut i = 0;
    loop {
        cycle(&mut rocks);
        let key = rocks.clone();
        let entry = rocks_cycles.entry(key);
        if let Occupied(first) = entry {
            cycle_length = i - first.get();
            break;
        }
        entry.or_insert(i);
        loads.push(calculate_load(&rocks));
        i += 1;
    }
    let first = i - cycle_length;
    let load = loads[(999999999 - first) % cycle_length + first];
    return format!("Total beam load: {}", load);
}

fn calculate_load(rocks: &Array2<u8>) -> usize {
    let mut load = 0;
    for col in rocks.columns() {
        for (i, &b) in col.iter().enumerate() {
            if b == b'O' {
                load += col.len() - i;
            }
        }
    }
    return load;
}

fn cycle(rocks: &mut Array2<u8>) {
    for _ in 0..4 {
        tilt_north(rocks);
        rotate_ccw(rocks);
    }
}

fn rotate_ccw(rocks: &mut Array2<u8>) {
    rocks.swap_axes(0, 1);
    rocks.invert_axis(Axis(1));
}

fn tilt_north(rocks: &mut Array2<u8>) {
    for mut col in rocks.columns_mut() {
        let mut target = 0;
        for source in 0..col.len() {
            if col[source] == b'#' {
                target = source + 1;
            }
            if col[source] == b'O' {
                col.swap(source, target);
                target += 1;
            }
        }
    }
}

fn parse_rocks(text: &str) -> Array2<u8> {
    let data = text.as_bytes();
    let width = data.iter().position(|&b| b == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);
    let shape = (height, width).strides((width + 1, 1));
    return Array::from_shape_vec(shape, data.into()).unwrap().into();
}
