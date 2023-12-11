pub fn part1(input: &str) -> String {
    let map = parse_map(input);
    let mut steps = 0;
    map.find_loop(|_| steps += 1);
    steps /= 2;
    return format!("Steps to farthest position: {}", steps);
}

pub fn part2(input: &str) -> String {
    let map = parse_map(input);
    let mut path = vec!();
    map.find_loop(|position| path.push(position));
    path.push(path[0]);
    path.push(path[1]);
    let mut spin = 0;
    for window in path.windows(3) {
        let &[a, b, c] = window else { panic!() };
        let ab = map.delta(a, b);
        let bc = map.delta(b, c);
        spin += ab.0 * bc.1 - ab.1 * bc.0;
    }
    spin /= 4;
    let mut visited = Vec::with_capacity(map.pipes.len());
    visited.resize(map.pipes.len(), false);
    let mut stack = vec!();
    for window in path.windows(3) {
        let &[a, b, c] = window else { panic!() };
        let ab = map.delta(a, b);
        let bc = map.delta(b, c);
        let turn = ab.0 * bc.1 - ab.1 * bc.0;
        if turn == spin {
            stack.push(map.shift(a, bc.0, bc.1));
        }
        else if turn == -spin {
            stack.push(map.shift(b, ab.0, ab.1));
            stack.push(map.shift(b, -bc.0, -bc.1));
        }
        visited[b] = true;
    }
    let mut area = 0;
    while let Some(i) = stack.pop() {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        area += 1;
        for (x, y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            stack.push(map.shift(i, x, y));
        }
    }
    return format!("Enclosed tiles: {}", area);
}

fn parse_map(text: &str) -> Map {
    let width = text.find('\n').unwrap() + 2;
    let height = (text.len() + 1) / (width + 1) + 2;
    let mut pipes = Vec::with_capacity(width * height);
    let mut start = 0;
    pipes.extend((0..=width).map(|i| (i, i)));
    for ch in text.chars() {
        let i = pipes.len();
        let pipe = match ch {
            '|' => (i - width, i + width),
            '-' => (i - 1, i + 1),
            'L' => (i - width, i + 1),
            'J' => (i - width, i - 1),
            '7' => (i + width, i - 1),
            'F' => (i + width, i + 1),
            '.' => (i, i),
            'S' => {
                start = pipes.len();
                (i, i)
            },
            '\n' => {
                pipes.push((i, i));
                (i + 1, i + 1)
            },
            _ => panic!(),
        };
        pipes.push(pipe);
    }
    pipes.extend((pipes.len()..pipes.capacity()).map(|i| (i, i)));
    return Map { width, pipes, start };
}

struct Map {
    width: usize,
    pipes: Vec<(usize, usize)>,
    start: usize,
}

impl Map {
    fn find_loop<F>(&self, mut update: F)
    where
        F: FnMut(usize),
    {
        let mut position = self.start;
        let mut previous = position;
        for (x, y) in [(1, 0), (0, 1), (-1, 0)] {
            let i = self.shift(position, x, y);
            if self.is_connected(i, position) {
                position = i;
                break;
            }
        }
        update(position);
        while position != self.start {
            let pipe = self.pipes[position];
            let next = if pipe.0 != previous { pipe.0 } else { pipe.1 };
            previous = position;
            position = next;
            update(position);
        }
    }

    fn is_connected(&self, from: usize, to: usize) -> bool {
        return self.pipes[from].0 == to || self.pipes[from].1 == to;
    }

    fn shift(&self, position: usize, x: isize, y: isize) -> usize {
        return (position as isize + x + y * self.width as isize) as usize;
    }

    fn delta(&self, from: usize, to: usize) -> (isize, isize) {
        let difference = to as isize - from as isize;
        return (difference % self.width as isize, difference / self.width as isize);
    }
}
