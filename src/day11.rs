pub fn part1(input: &str) -> String {
    let galaxies = parse_galaxies(input, 1);
    let sum = add_distances(galaxies);
    return format!("Sum of lengths: {}", sum);
}

pub fn part2(input: &str) -> String {
    let galaxies = parse_galaxies(input, 999999);
    let sum = add_distances(galaxies);
    return format!("Sum of lengths: {}", sum);
}

fn add_distances(galaxies: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for (i, (r1, c1)) in galaxies.iter().enumerate() {
        for (r2, c2) in &galaxies[i + 1..] {
            sum += r2 - r1 + std::cmp::max(c1, c2) - std::cmp::min(c1, c2);
        }
    }
    return sum;
}

fn parse_galaxies(text: &str, expansion: usize) -> Vec<(usize, usize)> {
    let mut added_cols = vec!();
    for line in text.lines() {
        for (c, ch) in line.chars().enumerate() {
            let galaxy = ch == '#';
            if let Some(galaxy_col) = added_cols.get_mut(c) {
                *galaxy_col *= !galaxy as usize;
            }
            else {
                added_cols.push(!galaxy as usize);
            }
        }
    }
    let mut galaxies = vec!();
    let mut new_r = 0;
    for line in text.lines() {
        let mut added_row = expansion;
        new_r += 1;
        let mut new_c = 0;
        for (c, ch) in line.chars().enumerate() {
            new_c += 1 + added_cols[c] * expansion;
            if ch == '#' {
                galaxies.push((new_r, new_c));
                added_row = 0;
            }
        }
        new_r += added_row;
    }
    return galaxies;
}
