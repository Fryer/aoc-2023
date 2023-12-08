pub fn part1(input: &str) -> String {
    let (seeds_text, maps_text) = input.split_once("\n\n").unwrap();
    let seeds = parse_seeds(seeds_text);
    let maps = parse_maps(maps_text);
    let mut locations: Vec<i64> = seeds.collect();
    for map in maps {
        let mapping = locations.iter().map(|&number| {
            for &SeedMapRange { destination, source, length } in &map.ranges {
                if number >= source && number < source + length {
                    return number - source + destination;
                }
            }
            return number;
        });
        locations = mapping.collect();
    }
    let lowest = locations.iter().min().unwrap();
    return format!("Lowest location: {}", lowest);
}

pub fn part2(input: &str) -> String {
    let (seeds_text, maps_text) = input.split_once("\n\n").unwrap();
    let seeds = parse_seed_ranges(seeds_text);
    let maps = parse_maps(maps_text);
    let mut locations: Vec<SeedRange> = seeds.collect();
    for mut map in maps {
        map.ranges.sort_by_key(|x| x.source);
        let mut mapping: Vec<SeedRange> = vec!();
        for mut seed in locations {
            for &SeedMapRange { destination, source, length } in &map.ranges {
                if seed.length == 0 || seed.start + seed.length <= source {
                    break;
                }
                if seed.start - source >= length {
                    continue;
                }
                if seed.start < source {
                    let cut = source - seed.start;
                    mapping.push(SeedRange {
                        start: seed.start,
                        length: cut
                    });
                    seed.start = source;
                    seed.length -= cut;
                }
                let cut = std::cmp::min(
                    seed.length,
                    source + length - seed.start
                );
                mapping.push(SeedRange {
                    start: seed.start - source + destination,
                    length: cut
                });
                seed.start += cut;
                seed.length -= cut;
            }
            if seed.length > 0 {
                mapping.push(seed);
            }
        }
        locations = mapping;
    }
    let lowest = locations.iter().map(|x| x.start).min().unwrap();
    return format!("Lowest location: {}", lowest);
}

fn parse_seeds(text: &str) -> impl Iterator<Item = i64> + '_ {
    let numbers_text = text.split_once(": ").unwrap().1;
    let number_texts = numbers_text.split(' ');
    return number_texts.map(|x| x.parse().unwrap());
}

fn parse_seed_ranges(text: &str) -> impl Iterator<Item = SeedRange> + '_ {
    let numbers_text = text.split_once(": ").unwrap().1;
    let number_texts = numbers_text.split(' ');
    let numbers = number_texts.map(|x| x.parse().unwrap());
    let range_tuples = numbers.clone().zip(numbers.skip(1)).step_by(2);
    return range_tuples.map(|(start, length)| SeedRange { start, length });
}

fn parse_maps(text: &str) -> impl Iterator<Item = SeedMap> + '_ {
    let map_texts = text.split("\n\n");
    return map_texts.map(parse_map);
}

fn parse_map(text: &str) -> SeedMap {
    let range_texts = text.lines().skip(1);
    let ranges = range_texts.map(parse_range);
    return SeedMap { ranges: ranges.collect() };
}

fn parse_range(text: &str) -> SeedMapRange {
    let mut numbers = text.split(' ').map(|x| x.parse().unwrap());
    let destination = numbers.next().unwrap();
    let source = numbers.next().unwrap();
    let length = numbers.next().unwrap();
    return SeedMapRange { destination, source, length };
}

struct SeedRange {
    start: i64,
    length: i64,
}

struct SeedMap {
    ranges: Vec<SeedMapRange>,
}

struct SeedMapRange {
    destination: i64,
    source: i64,
    length: i64,
}
