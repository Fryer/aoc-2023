use itertools::Itertools;

pub fn part1(input: &str) -> String {
    const MIN: i128 = 200000000000000;
    const MAX: i128 = 400000000000000;
    let hailstones = parse_hailstones(input);
    let mut intersections = 0;
    for (a, b) in hailstones.iter().tuple_combinations() {
        if let Some((ix, iy)) = intersect(a, b) {
            let x_inside = ix.0 >= MIN * ix.1 && ix.0 <= MAX * ix.1;
            let y_inside = iy.0 >= MIN * iy.1 && iy.0 <= MAX * iy.1;
            if x_inside && y_inside {
                intersections += 1;
            }
        }
    }
    return format!("Intersections: {}", intersections);
}

pub fn part2(input: &str) -> String {
    let hailstones = parse_hailstones(input);
    let mut velocity = (0i128, 0i128);
    let mut position = (0, 0, 0);
    loop {
        let expand = velocity.0 < 0 && velocity.0 == -velocity.1;
        if velocity.0.abs() > velocity.1.abs() || expand {
            velocity.1 += if velocity.0 < 0 { -1 } else { 1 };
        }
        else {
            velocity.0 += if velocity.1 < 0 { 1 } else { -1 };
        }
        let reframed = hailstones.iter()
            .map(|h| Hailstone {
                p: h.p,
                v: (h.v.0 - velocity.0, h.v.1 - velocity.1, h.v.2)
            });
        let (a, b, c) = reframed.take(3).collect_tuple().unwrap();
        if let (Some((abx, aby)), Some((acx, acy))) = (intersect(&a, &b), intersect(&a, &c)) {
            if abx.0 * acx.1 == acx.0 * abx.1 && aby.0 * acy.1 == acy.0 * aby.1 {
                position.0 = abx.0 / abx.1;
                position.1 = aby.0 / aby.1;
                let at = (position.0 - a.p.0) / a.v.0;
                let bt = (position.0 - b.p.0) / b.v.0;
                let aiz = a.p.2 + a.v.2 * at;
                let biz = b.p.2 + b.v.2 * bt;
                let vz = (biz - aiz) / (bt - at);
                position.2 = aiz - vz * at;
                break;
            }
        }
    }
    let sum = position.0 + position.1 + position.2;
    return format!("Sum of coordinates: {}", sum);
}

fn intersect(a: &Hailstone, b: &Hailstone) -> Option<((i128, i128), (i128, i128))> {
    let d = a.v.0 * b.v.1 - a.v.1 * b.v.0;
    if d == 0 {
        return None;
    }
    let c1 = b.p.0 * b.v.1 - b.p.1 * b.v.0;
    let c2 = a.p.0 * a.v.1 - a.p.1 * a.v.0;
    let ix = (c1 * a.v.0 - c2 * b.v.0, d);
    let iy = (c1 * a.v.1 - c2 * b.v.1, d);
    let ix = (ix.0.abs(), ix.1 * ix.0.signum());
    let iy = (iy.0.abs(), iy.1 * iy.0.signum());
    let a_future = (ix.0 - a.p.0 * ix.1).signum() == a.v.0.signum();
    let b_future = (ix.0 - b.p.0 * ix.1).signum() == b.v.0.signum();
    if a_future && b_future {
        return Some((ix, iy));
    }
    return None;
}

fn parse_hailstones(text: &str) -> Vec<Hailstone> {
    let mut hailstones = vec!();
    for line in text.lines() {
        let (position, velocity) = line.split_once('@').unwrap();
        let p = position.split(',')
            .map(|x| x.trim().parse::<i128>().unwrap())
            .collect_tuple::<(_, _, _)>().unwrap();
        let v = velocity.split(',')
            .map(|x| x.trim().parse::<i128>().unwrap())
            .collect_tuple::<(_, _, _)>().unwrap();
        hailstones.push(Hailstone { p, v });
    }
    return hailstones;
}

struct Hailstone {
    p: (i128, i128, i128),
    v: (i128, i128, i128),
}
