use ndarray::{Array, Array2, ArrayView, ArrayView2, ShapeBuilder};

pub fn part1(input: &str) -> String {
    let data = input.as_bytes();
    let width = data.iter().position(|&b| b == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);
    let shape = (height, width).strides((width + 1, 1));
    let grid = ArrayView::from_shape(shape, data).unwrap();
    let mut visited = Array::from_elem((height, width), false);
    visited[(0, 1)] = true;
    let longest = step(&grid, &mut visited, 1, 1, 1);
    return format!("Longest hike: {}", longest);
}

pub fn part2(input: &str) -> String {
    let data = input.as_bytes();
    let width = data.iter().position(|&b| b == b'\n').unwrap();
    let height = (data.len() + 1) / (width + 1);
    let shape = (height, width).strides((width + 1, 1));
    let grid = ArrayView::from_shape(shape, data).unwrap();
    let mut tree = vec![Node::new([None; 4]); 2];
    let mut node_map = Array::from_elem((height, width), None);
    node_map[(0, 1)] = Some((0, 1));
    node_map[(height - 1, width - 2)] = Some((1, 1));
    for r in 1..height - 1 {
        for c in 1..width - 1 {
            if grid[(r, c)] == b'#' {
                continue;
            }
            node_map[(r, c)] = Some((tree.len(), 1));
            tree.push(Node::new([None; 4]));
        }
    }
    tree[0].edges[3] = node_map[(1, 1)];
    tree[1].edges[3] = node_map[(height - 2, width - 2)];
    for r in 1..height - 1 {
        for c in 1..width - 1 {
            let i = node_map[(r, c)];
            if i.is_none() {
                continue;
            }
            let i = i.unwrap().0;
            tree[i] = Node::new([
                node_map[(r + 1, c)],
                node_map[(r, c + 1)],
                node_map[(r - 1, c)],
                node_map[(r, c - 1)],
            ]);
        }
    }
    for i in 2..tree.len() {
        if let [None, None, Some((j, j_length)), Some((k, k_length))] = tree[i].edges {
            let length = j_length + k_length;
            tree[j].change_edge(i, (k, length));
            tree[k].change_edge(i, (j, length));
            tree[i].edges = [None; 4];
        }
    }
    for turn in [1, -1] {
        let mut last_i = 0;
        let (mut r, mut c) = (1, 1);
        let (mut dr, mut dc) = (1, 0);
        while r + 1 != height {
            let i = node_map[(r, c)].unwrap().0;
            tree[i].remove_edge(last_i);
            let neighbors = [(-dc * turn, dr * turn), (dr, dc), (dc * turn, -dr * turn)];
            for (next_dr, next_dc) in neighbors {
                let next = (r.wrapping_add_signed(next_dr), c.wrapping_add_signed(next_dc));
                if grid[next] != b'#' {
                    (r, c) = next;
                    (dr, dc) = (next_dr, next_dc);
                    break;
                }
            }
            if tree[i].edges[3].is_some() {
                last_i = i;
            }
        }
    }
    let mut visited = vec![false; tree.len()];
    let longest = step_tree(&tree, &mut visited, 0, 0);
    return format!("Longest hike: {}", longest);
}

fn step(
    grid: &ArrayView2<u8>,
    visited: &mut Array2<bool>,
    r: usize, c: usize, length: usize,
) -> usize {
    if visited[(r, c)] || grid[(r, c)] == b'#' {
        return 0;
    }
    if r == grid.nrows() - 1 {
        return length;
    }
    visited[(r, c)] = true;
    let mut longest = 0;
    if grid[(r + 1, c)] != b'^' {
        longest = step(grid, visited, r + 1, c, length + 1);
    }
    if grid[(r, c + 1)] != b'<' {
        longest = longest.max(step(grid, visited, r, c + 1, length + 1));
    }
    if grid[(r - 1, c)] != b'v' {
        longest = longest.max(step(grid, visited, r - 1, c, length + 1));
    }
    if grid[(r, c - 1)] != b'>' {
        longest = longest.max(step(grid, visited, r, c - 1, length + 1));
    }
    visited[(r, c)] = false;
    return longest;
}

fn step_tree(
    tree: &Vec<Node>,
    visited: &mut Vec<bool>,
    i: usize, length: usize,
) -> usize {
    if visited[i] {
        return 0;
    }
    if i == 1 {
        return length;
    }
    visited[i] = true;
    let mut longest = 0;
    for edge in tree[i].edges {
        if let Some((j, j_length)) = edge {
            longest = longest.max(step_tree(tree, visited, j, length + j_length));
        }
    }
    visited[i] = false;
    return longest;
}

#[derive(Clone)]
struct Node {
    edges: [Option<(usize, usize)>; 4],
}

impl Node {
    fn new(mut edges: [Option<(usize, usize)>; 4]) -> Self {
        edges.sort();
        return Self { edges };
    }

    fn remove_edge(&mut self, i: usize) {
        for p in 0..4 {
            if self.edges[p].is_some_and(|(j, _)| j == i) {
                self.edges[p] = None;
                self.edges.sort();
            }
        }
    }

    fn change_edge(&mut self, i: usize, new_edge: (usize, usize)) {
        for edge in &mut self.edges {
            if let Some(edge) = edge {
                if edge.0 == i {
                    *edge = new_edge;
                }
            }
        }
    }
}
