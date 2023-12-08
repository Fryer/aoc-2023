use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let mut grid: Vec<Vec<_>> = input.lines().map(|x| x.chars().collect()).collect();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' || ch.is_numeric() {
                continue;
            }
            for (ny, nx) in NEIGHBORS {
                if let Some(nline) = grid.get_mut(y.wrapping_add_signed(*ny)) {
                    if let Some(nch) = nline.get_mut(x.wrapping_add_signed(*nx)) {
                        *nch = '#';
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        let mut number = String::new();
        let mut part = false;
        for (x, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                number.push(ch);
                if grid[y][x] == '#' {
                    part = true;
                }
                continue;
            }
            if part {
                sum += number.parse::<u32>().unwrap();
                part = false;
            }
            number.clear();
        }
        if part {
            sum += number.parse::<u32>().unwrap();
        }
    }
    return format!("Sum of engine parts: {}", sum);
}

pub fn part2(input: &str) -> String {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut grid = Grid::new(height, width);
    let mut gears: Vec<Gear> = vec!();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '*' {
                continue;
            }
            let gear_id = gears.len();
            for (dy, dx) in NEIGHBORS {
                let ny = y.wrapping_add_signed(*dy);
                let nx = x.wrapping_add_signed(*dx);
                if let Some(cell) = grid.get_mut(ny, nx) {
                    cell.gear_ids.push(gear_id);
                }
            }
            gears.push(Gear { numbers: vec!() });
        }
    }

    for (y, line) in input.lines().enumerate() {
        let mut number = String::new();
        let mut gear_ids = HashSet::<usize>::new();
        for (x, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                number.push(ch);
                gear_ids.extend(&grid.get(y, x).unwrap().gear_ids);
                continue;
            }
            for gear_id in &gear_ids {
                gears[*gear_id].numbers.push(number.parse().unwrap());
            }
            gear_ids.clear();
            number.clear();
        }
        for gear_id in gear_ids {
            gears[gear_id].numbers.push(number.parse().unwrap());
        }
    }

    let mut sum = 0;
    for gear in gears {
        if gear.numbers.len() == 2 {
            sum += gear.numbers.iter().product::<u32>();
        }
    }
    return format!("Sum of gear ratios: {}", sum);
}

const NEIGHBORS: &[(isize, isize)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 0), (0, 1),
    (1, -1), (1, 0), (1, 1),
];

struct Grid {
    rows: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(height: usize, width: usize) -> Grid {
        let cell = Cell { gear_ids: vec!() };
        let row = vec!(cell; width);
        let grid = Grid { rows: vec!(row; height) };
        return grid;
    }

    fn get(&self, y: usize, x: usize) -> Option<&Cell> {
        if let Some(row) = self.rows.get(y) {
            if let Some(cell) = row.get(x) {
                return Some(cell);
            }
        }
        return None;
    }

    fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut Cell> {
        if let Some(row) = self.rows.get_mut(y) {
            if let Some(cell) = row.get_mut(x) {
                return Some(cell);
            }
        }
        return None;
    }
}

#[derive(Clone)]
struct Cell {
    gear_ids: Vec<usize>,
}

struct Gear {
    numbers: Vec<u32>,
}
