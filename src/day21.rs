use std::collections::VecDeque;
use ndarray::{Array2, ArrayView, ShapeBuilder};
use num_integer::Integer;

pub fn part1(input: &str) -> String {
    let data = input.as_bytes();
    let mut garden = Garden::new(&data, true);
    for _ in 0..=64 {
        garden.step();
    }
    return format!("Reachable plots: {}", garden.reachable_at(64));
}

pub fn part2(input: &str) -> String {
    const STEPS: usize = 26501365;
    let data = input.as_bytes();
    let mut garden = Garden::new(&data, true);
    let (start_r, start_c) = garden.queue[0];
    while !garden.queue.is_empty() {
        garden.step();
    }
    let reachable_inner = garden.reachable_at(STEPS);
    let reachable_odd = garden.reachable_at(STEPS + 1);
    let mut reachable = reachable_inner;
    let size = garden.grid.nrows();
    let steps_to_first_corner = size - 1;
    let steps_to_first_edge = steps_to_first_corner / 2;
    let inner_edges = (STEPS - steps_to_first_edge - 1) / size - 1;
    let steps_after_last_edge = (STEPS - steps_to_first_edge - 1) % size;
    reachable += reachable_inner * (inner_edges / 2) * 4;
    reachable += reachable_odd * ((inner_edges + 1) / 2) * 4;
    let inner_corners = (STEPS - steps_to_first_corner - 2) / size - 1;
    let steps_after_last_corner = (STEPS - steps_to_first_corner - 2) % size;
    let even_corners = (inner_corners + 1) / 2;
    let odd_corners = inner_corners / 2;
    reachable += reachable_inner * even_corners * even_corners * 4;
    reachable += reachable_odd * (odd_corners * odd_corners + odd_corners) * 4;
    let steps_after_penultimate_edge = steps_after_last_edge + size;
    let steps_after_penultimate_corner = steps_after_last_corner + size;
    for start in [(0, start_c), (start_r, 0), (size - 1, start_c), (start_r, size - 1)] {
        let mut garden = Garden::new(&data, false);
        garden.queue.push_back(start);
        while !garden.queue.is_empty() && garden.steps <= steps_after_last_edge {
            garden.step();
        }
        reachable += garden.reachable_at(steps_after_last_edge);
        while !garden.queue.is_empty() && garden.steps <= steps_after_penultimate_edge {
            garden.step();
        }
        reachable += garden.reachable_at(steps_after_penultimate_edge);
    }
    for start in [(0, 0), (size - 1, 0), (size - 1, size - 1), (0, size - 1)] {
        let mut garden = Garden::new(&data, false);
        garden.queue.push_back(start);
        while !garden.queue.is_empty() && garden.steps <= steps_after_last_corner {
            garden.step();
        }
        reachable += garden.reachable_at(steps_after_last_corner) * (inner_corners + 2);
        while !garden.queue.is_empty() && garden.steps <= steps_after_penultimate_corner {
            garden.step();
        }
        reachable += garden.reachable_at(steps_after_penultimate_corner) * (inner_corners + 1);
    }
    return format!("Reachable plots: {}", reachable);
}

struct Garden {
    grid: Array2<usize>,
    queue: VecDeque<(usize, usize)>,
    steps: usize,
    reachable: (usize, usize),
}

impl Garden {
    fn new(data: &[u8], place_start: bool) -> Self {
        let width = data.iter().position(|&b| b == b'\n').unwrap();
        let height = (data.len() + 1) / (width + 1);
        let shape = (height, width).strides((width + 1, 1));
        let grid = ArrayView::from_shape(shape, data).unwrap().map(|&b| match b {
            b'#' => 0,
            b'.' | b'S' => usize::MAX,
            _ => panic!(),
        });
        let mut queue = VecDeque::new();
        if place_start {
            let start = data.iter().position(|&b| b == b'S').unwrap().div_rem(&(width + 1));
            queue.push_back(start);
        }
        return Self {
            grid,
            queue,
            steps: 0,
            reachable: (0, 0),
        };
    }

    fn step(&mut self) {
        self.reachable = (self.reachable.1, self.reachable.0);
        let mut remaining = self.queue.len();
        while remaining > 0 {
            let (r, c) = self.queue.pop_front().unwrap();
            remaining -= 1;
            if self.grid[(r, c)] <= self.steps {
                continue;
            }
            self.grid[(r, c)] = self.steps;
            self.reachable.0 += 1;
            if r > 0 {
                self.queue.push_back((r - 1, c));
            }
            if c > 0 {
                self.queue.push_back((r, c - 1));
            }
            if r + 1 < self.grid.nrows() {
                self.queue.push_back((r + 1, c));
            }
            if c + 1 < self.grid.ncols() {
                self.queue.push_back((r, c + 1));
            }
        }
        self.steps += 1;
    }

    fn reachable_at(&self, step: usize) -> usize {
        return if self.steps % 2 == step % 2 { self.reachable.1 } else { self.reachable.0 };
    }
}
