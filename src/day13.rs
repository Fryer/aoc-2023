use ndarray::{ArrayView, ArrayView2, Ix1, ShapeBuilder};
use ndarray::iter::Lanes;

pub fn part1(input: &str) -> String {
    let patterns = parse_patterns(input);
    let mut sum = 0;
    for pattern in patterns {
        if let Some(mirror) = find_mirror(pattern.rows()) {
            sum += 100 * mirror;
        }
        else if let Some(mirror) = find_mirror(pattern.columns()) {
            sum += mirror;
        }
    }
    return format!("Sum of notes: {}", sum);
}

pub fn part2(input: &str) -> String {
    let patterns = parse_patterns(input);
    let mut sum = 0;
    for pattern in patterns {
        if let Some(mirror) = find_smudged_mirror(pattern.rows()) {
            sum += 100 * mirror;
        }
        else if let Some(mirror) = find_smudged_mirror(pattern.columns()) {
            sum += mirror;
        }
    }
    return format!("Sum of notes: {}", sum);
}

fn find_mirror(lanes: Lanes<u8, Ix1>) -> Option<usize> {
    let codes_iter = lanes.into_iter().map(|row|
        row.iter().fold(0, |code, &b| (code << 1) + (b == b'#') as u32)
    );
    let codes: Vec<_> = codes_iter.collect();
    let mut mirrors = vec!(0; codes.len() - 1);
    let mut i = 0;
    while i < mirrors.len() {
        let mirror = mirrors[i];
        let left = if i < mirror { None } else { Some(codes[i - mirror]) };
        let right = codes.get(i + mirror + 1).copied();
        if left.is_some() && left == right {
            mirrors[i] += 1;
            continue;
        }
        if mirror > i || i + mirror == mirrors.len() {
            return Some(i + 1);
        }
        i += 1;
    }
    return None;
}

fn find_smudged_mirror(lanes: Lanes<u8, Ix1>) -> Option<usize> {
    let codes_iter = lanes.into_iter().map(|row|
        row.iter().fold(0, |code, &b| (code << 1) + (b == b'#') as u32)
    );
    let codes: Vec<_> = codes_iter.collect();
    let mut mirrors = vec!(0; codes.len() - 1);
    let mut smudges = vec!(false; codes.len() - 1);
    let mut i = 0;
    while i < mirrors.len() {
        let mirror = mirrors[i];
        let left = if i < mirror { None } else { Some(codes[i - mirror]) };
        let right = codes.get(i + mirror + 1).copied();
        if let Some(smudge) = smudged_eq(left, right) {
            if !(smudge && smudges[i]) {
                mirrors[i] += 1;
                smudges[i] |= smudge;
                continue;
            }
        }
        if smudges[i] && (mirror > i || i + mirror == mirrors.len()) {
            return Some(i + 1);
        }
        i += 1;
    }
    return None;
}

fn smudged_eq(a: Option<u32>, b: Option<u32>) -> Option<bool> {
    if a.is_none() || b.is_none() {
        return None;
    }
    let smudge = (a.unwrap() ^ b.unwrap()).count_ones();
    if smudge > 1 {
        return None;
    }
    return Some(smudge == 1);
}

fn parse_patterns(text: &str) -> Vec<ArrayView2<u8>> {
    let pattern_texts = text.split("\n\n");
    let patterns = pattern_texts.map(parse_pattern);
    return patterns.collect();
}

fn parse_pattern(text: &str) -> ArrayView2<u8> {
    let width = text.find('\n').unwrap();
    let height = (text.len() + 1) / (width + 1);
    let shape = (height, width).strides((width + 1, 1));
    let pattern = ArrayView::from_shape(shape, text.as_bytes()).unwrap();
    return pattern;
}
