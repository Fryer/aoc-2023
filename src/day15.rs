pub fn part1(input: &str) -> String {
    let data = input.as_bytes();
    let steps = data.split(|&b| b == b',');
    let mut sum = 0;
    for step in steps {
        sum += hash_slice(step) as u32;
    }
    return format!("Sum of results: {}", sum);
}

pub fn part2(input: &str) -> String {
    let data = input.as_bytes();
    let steps = data.split(|&b| b == b',');
    let mut boxes: Vec<Vec<(&[u8], u8)>> = vec!(vec!(); 256);
    for step in steps {
        let cut = step.iter().position(|&b| b == b'-' || b == b'=').unwrap();
        let label = &step[0..cut];
        let mut _box = &mut boxes[hash_slice(label) as usize];
        let slot = _box.iter().position(|(l, _)| *l == label);
        let operation = step[cut];
        match operation {
            b'=' => {
                let focal_length = step[cut + 1] - b'0';
                match slot {
                    Some(slot) => _box[slot] = (label, focal_length),
                    None => _box.push((label, focal_length)),
                };
            },
            b'-' => {
                if let Some(slot) = slot {
                    _box.remove(slot);
                }
            },
            _ => panic!(),
        };
    }
    let mut power = 0;
    for (i, _box) in boxes.iter().enumerate() {
        for (j, &lens) in _box.iter().enumerate() {
            let (_, focal_length) = lens;
            power += (i + 1) * (j + 1) * focal_length as usize;
        }
    }
    return format!("Focusing power: {}", power);
}

fn hash_slice(slice: &[u8]) -> u8 {
    return slice.iter().copied().fold(0, hash_byte);
}

fn hash_byte(hash: u8, byte: u8) -> u8 {
    return hash.wrapping_add(byte).wrapping_mul(17);
}
