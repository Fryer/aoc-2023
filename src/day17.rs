use std::cmp::Ordering;
use std::collections::BinaryHeap;
use ndarray::{Array, ArrayView, s, ShapeBuilder};

pub fn part1(input: &str) -> String {
    let lowest = calculate_lowest_heat(input, 1, 3);
    return format!("Lowest heat loss: {}", lowest);
}

pub fn part2(input: &str) -> String {
    let lowest = calculate_lowest_heat(input, 4, 10);
    return format!("Lowest heat loss: {}", lowest);
}

fn calculate_lowest_heat(input: &str, min_step: usize, max_step: usize) -> usize {
    let data = input.as_bytes();
    let w = data.iter().position(|&b| b == b'\n').unwrap();
    let h = (data.len() + 1) / (w + 1);
    let shape = (h, w).strides((w + 1, 1));
    let drains = ArrayView::from_shape(shape, data).unwrap();
    let mut losses = Array::from_elem((h, w, 4 * max_step), usize::MAX);
    let mut queue = BinaryHeap::from([
        Crucible::new(0, 0, 1, 0, 1),
        Crucible::new(0, 1, 0, 1, 0),
    ]);
    while !queue.is_empty() {
        let Crucible { mut loss, r, c, dr, dc } = queue.pop().unwrap();
        loss += (drains[(r, c)] - b'0') as usize;
        if (dr + dc).abs() > max_step as i8 {
            continue;
        }
        if (dr + dc).abs() >= min_step as i8 {
            let dcode = match (dr, dc) {
                (dr, _) if dr < 0 => -1 - dr,
                (_, dc) if dc < 0 => max_step as i8 - 1 - dc,
                (dr, _) if dr > 0 => 2 * max_step as i8 - 1 + dr,
                (_, dc) if dc > 0 => 3 * max_step as i8 - 1 + dc,
                _ => panic!(),
            } as usize;
            if losses[(r, c, dcode)] <= loss {
                continue;
            }
            let slice = s![r, c, dcode..dcode + max_step - dcode % max_step];
            for other in &mut losses.slice_mut(slice) {
                *other = loss.min(*other);
            }
        }
        if r > 0 && (dr < 0 || dc.abs() >= min_step as i8) {
            queue.push(Crucible { loss, r: r - 1, c, dr: dr - 1, dc: 0 });
        }
        if c > 0 && (dc < 0 || dr.abs() >= min_step as i8) {
            queue.push(Crucible { loss, r, c: c - 1, dr: 0, dc: dc - 1 });
        }
        if r + 1 < h && (dr > 0 || dc.abs() >= min_step as i8) {
            queue.push(Crucible { loss, r: r + 1, c, dr: dr + 1, dc: 0 });
        }
        if c + 1 < w && (dc > 0 || dr.abs() >= min_step as i8) {
            queue.push(Crucible { loss, r, c: c + 1, dr: 0, dc: dc + 1 });
        }
    }
    return *losses.slice(s![-1, -1, ..]).iter().min().unwrap();
}

#[derive(Eq)]
struct Crucible {
    loss: usize,
    r: usize,
    c: usize,
    dr: i8,
    dc: i8,
}

impl Crucible {
    fn new(loss: usize, r: usize, c: usize, dr: i8, dc: i8) -> Crucible {
        return Crucible { loss, r, c, dr, dc };
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.loss.cmp(&other.loss).reverse();
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Crucible {
    fn eq(&self, other: &Self) -> bool {
        return self.loss == other.loss;
    }
}
