use std::mem::swap;
use itertools::Itertools;
use ndarray::{Array, Array2};

pub fn part1(input: &str) -> String {
    let (mut bricks, mut grid) = parse_bricks(input);
    let mut brick_safety = vec!(1; bricks.len());
    for i in 1..bricks.len() {
        let ((x1, y1, z1), (x2, y2, _)) = bricks[i];
        let mut support = Some(grid[(x1, y1)]);
        let mut z = bricks[grid[(x1, y1)]].1.2;
        for x in x1..=x2 {
            for y in y1..=y2 {
                let j = grid[(x, y)];
                let brick = bricks[j];
                if brick.1.2 > z {
                    support = Some(j);
                    z = brick.1.2;
                }
                if brick.1.2 == z && Some(j) != support {
                    support = None;
                }
                grid[(x, y)] = i;
            }
        }
        if let Some(j) = support {
            brick_safety[j] = 0;
        }
        bricks[i].1.2 -= z1 - z - 1;
        bricks[i].0.2 = z + 1;
    }
    let safe_bricks: u32 = brick_safety.iter().sum();
    return format!("Disintegratable bricks: {}", safe_bricks);
}

pub fn part2(input: &str) -> String {
    let (mut bricks, mut grid) = parse_bricks(input);
    let mut supports = vec!(0; bricks.len());
    let mut heights = vec!(0; bricks.len());
    for i in 1..bricks.len() {
        let ((x1, y1, z1), (x2, y2, _)) = bricks[i];
        let mut support = grid[(x1, y1)];
        let mut z = bricks[grid[(x1, y1)]].1.2;
        for x in x1..=x2 {
            for y in y1..=y2 {
                let mut j = grid[(x, y)];
                let brick = bricks[j];
                if brick.1.2 > z {
                    support = j;
                    z = brick.1.2;
                }
                if brick.1.2 == z {
                    while j != support {
                        if bricks[j].1.2 > bricks[support].1.2 {
                            j = supports[j];
                        }
                        else {
                            support = supports[support];
                        }
                    }
                }
                grid[(x, y)] = i;
            }
        }
        supports[i] = support;
        heights[i] = heights[support] + 1;
        bricks[i].1.2 -= z1 - z - 1;
        bricks[i].0.2 = z + 1;
    }
    let falling_bricks = heights.iter().sum::<usize>() + 1 - bricks.len();
    return format!("Sum of falling bricks: {}", falling_bricks);
}

fn parse_bricks(text: &str) -> (Vec<Brick>, Grid) {
    let mut bricks = vec!();
    let mut xy_max = (0, 0);
    for line in text.lines() {
        let (start_text, end_text) = line.split_once('~').unwrap();
        let start_nums = start_text.split(',').map(|x| x.parse().unwrap());
        let end_nums = end_text.split(',').map(|x| x.parse().unwrap());
        let mut start: (_, _, _) = start_nums.collect_tuple().unwrap();
        let mut end: (_, _, _) = end_nums.collect_tuple().unwrap();
        if start.2 > end.2 {
            swap(&mut start, &mut end);
        }
        xy_max = (xy_max.0.max(start.0).max(end.0), xy_max.1.max(start.1).max(end.1));
        bricks.push((start, end));
    }
    xy_max.0 += 1;
    xy_max.1 += 1;
    bricks.push(((0, 0, 0), (xy_max.0, xy_max.1, 0)));
    bricks.sort_by_key(|x| x.0.2);
    let grid = Array::from_elem(xy_max, 0);
    return (bricks, grid);
}

type Grid = Array2<usize>;
type Brick = (Coord, Coord);
type Coord = (usize, usize, usize);
