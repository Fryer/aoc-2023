use std::collections::VecDeque;
use ndarray::{Array, Array2, ArrayView, ArrayView2, ShapeBuilder};
use crate::canvas::{Canvas, Color};

pub fn part1(input: &str) -> String {
    let grid = parse_grid(input);
    let energy = trace_beam(&grid, 0, 0, 0);
    return format!("Energized tiles: {}", energy);
}

pub fn part2(input: &str) -> String {
    let grid = parse_grid(input);
    let mut max = 0;
    let mut best_beam = (0, 0, 0);
    for r in 0..grid.nrows() {
        let energy = trace_beam(&grid, r as isize, 0, 0);
        if energy > max {
            max = energy;
            best_beam = (r, 0, 0);
        }
        let energy = trace_beam(&grid, r as isize, grid.ncols() as isize - 1, 2);
        if energy > max {
            max = energy;
            best_beam = (r, grid.ncols() - 1, 2);
        }
    }
    for c in 0..grid.ncols() {
        let energy = trace_beam(&grid, 0, c as isize, 1);
        if energy > max {
            max = energy;
            best_beam = (0, c, 1);
        }
        let energy = trace_beam(&grid, grid.nrows() as isize - 1, c as isize, 3);
        if energy > max {
            max = energy;
            best_beam = (grid.nrows() - 1, c, 3);
        }
    }
    return format!("Maximum energized tiles: {} {:?}", max, best_beam);
}

fn trace_beam(grid: &ArrayView2<u8>, r: isize, c: isize, d: u8) -> usize {
    let (mut visited, mut beams) = init_beams(grid, r, c, d);
    let mut energy = 0;
    while !beams.is_empty() {
        energy += step_beams(grid, &mut visited, &mut beams);
    }
    return energy;
}

fn init_beams(
    grid: &ArrayView2<u8>, r: isize, c: isize, d: u8
) -> (Array2<u8>, VecDeque<(isize, isize, u8)>) {
    let &[h, w] = grid.shape() else { panic!() };
    let visited = Array::from_elem((h, w), 0u8);
    let beams = VecDeque::from([(r, c, d)]);
    return (visited, beams);
}

fn step_beams(
    grid: &ArrayView2<u8>,
    visited: &mut Array2<u8>,
    beams: &mut VecDeque<(isize, isize, u8)>,
) -> usize {
    let &[h, w] = grid.shape() else { panic!() };
    let mut energy = 0;
    let mut queue = beams.len();
    while queue > 0 {
        queue -= 1;
        let (r, c, d) = beams.pop_front().unwrap();
        if r < 0 || c < 0 || r >= h as isize || c >= w as isize {
            continue;
        }
        if (visited[(r as usize, c as usize)] & (1 << d)) > 0 {
            continue;
        }
        if visited[(r as usize, c as usize)] == 0 {
            energy += 1;
        }
        visited[(r as usize, c as usize)] |= 1 << d;
        let b = grid[(r as usize, c as usize)];
        match (b, d) {
            (b'.' | b'-', 0) | (b'\\', 1) | (b'/', 3) => {
                beams.push_back((r, c + 1, 0));
            },
            (b'.' | b'|', 1) | (b'/', 2) | (b'\\', 0) => {
                beams.push_back((r + 1, c, 1));
            },
            (b'.' | b'-', 2) | (b'\\', 3) | (b'/', 1) => {
                beams.push_back((r, c - 1, 2));
            },
            (b'.' | b'|', 3) | (b'/', 0) | (b'\\', 2) => {
                beams.push_back((r - 1, c, 3));
            },
            (b'|', 0 | 2) => {
                beams.push_back((r + 1, c, 1));
                beams.push_back((r - 1, c, 3));
            },
            (b'-', 1 | 3) => {
                beams.push_back((r, c + 1, 0));
                beams.push_back((r, c - 1, 2));
            },
            _ => panic!(),
        }
    }
    return energy;
}

fn parse_grid(text: &str) -> ArrayView2<u8> {
    let data = text.as_bytes();
    let width = data.iter().position(|&b| b == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);
    let shape = (height, width).strides((width + 1, 1));
    return ArrayView::from_shape(shape, data).unwrap();
}

pub fn visualize(input: &str) -> String {
    let grid = parse_grid(input);
    let (mut visited, mut beams) = init_beams(&grid, 0, 62, 1);
    let mut canvas = Canvas::new();
    let mut energy = 0;
    while !beams.is_empty() {
        canvas.reset_style();
        canvas.begin(50);
        draw_grid(&mut canvas, &grid, &visited, &beams);
        canvas.end();
        energy += step_beams(&grid, &mut visited, &mut beams);
    }
    canvas.reset_style();
    canvas.begin(0);
    draw_grid(&mut canvas, &grid, &visited, &beams);
    let (th, tw) = canvas.size();
    let lines = [
        "",
        &format!("Energized tiles: {}", energy),
        "",
        "Press any key to continue...",
        "",
    ];
    let r = (th / 2).saturating_sub(lines.len() / 2);
    let c = (tw / 2).saturating_sub(15);
    canvas.style(Color::Black, (191, 191, 191));
    for (i, &line) in lines.iter().enumerate() {
        canvas.print(r + i, c, &format!("{:^30}", line));
    }
    canvas.end();
    canvas.pause();
    return format!("Energized tiles: {}", energy);
}

fn draw_grid(
    canvas: &mut Canvas,
    grid: &ArrayView2<u8>,
    visited: &Array2<u8>,
    beams: &VecDeque<(isize, isize, u8)>,
) {
    let &[h, w] = grid.shape() else { panic!() };
    let (th, tw) = canvas.size();
    let dr = th.saturating_sub(h) / 2;
    let dc = tw.saturating_sub(w) / 2;
    for ((r, c), &b) in grid.indexed_iter() {
        match b {
            b'.' => canvas.fg((64, 64, 64)),
            _ => canvas.fg((127, 127, 191)),
        }
        let power = visited[(r, c)].count_ones() as u8;
        match visited[(r, c)] {
            0 => canvas.bg((16, 16, 16)),
            _ => canvas.bg((16 + 32 * power, 16 + 32 * power, 16 + 16 * power)),
        }
        canvas.draw(dr + r, dc + c, b as char);
    }
    for &(r, c, _) in beams {
        if r < 0 || c < 0 || r >= h as isize || c >= w as isize {
            continue;
        }
        canvas.style(Color::White, (32, 32, 32));
        canvas.draw(dr + r as usize, dc + c as usize, '+');
    }
}
