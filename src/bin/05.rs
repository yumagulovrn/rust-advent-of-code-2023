advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    src_start: u64,
    dst_start: u64,
    length: u64
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let values: Vec<u64> = value.split_whitespace().filter_map(|v| v.parse::<u64>().ok()).collect();
        Map { dst_start: values[0], src_start: values[1], length: values[2] }
    }
}

fn convert(input: u64, mappings: &[Map]) -> u64 {
    for mapping in mappings {
        if input >= mapping.src_start && input < mapping.src_start + mapping.length {
            return mapping.dst_start + input - mapping.src_start;
        }
    }
    input
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Map>>) {
    let mut input = input.split("\n\n");
    let seeds: Vec<u64> = input.next().unwrap().split_whitespace().skip(1).map(|seed| seed.parse::<u64>().unwrap()).collect();
    let mappings: Vec<Vec<Map>> = input.map(|block| {
        block.lines().skip(1).map(|maps| Map::from(maps)).collect::<Vec<Map>>()
    }).collect();
    (seeds, mappings)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, mappings) = parse(input);
    Some(seeds.iter().map(|seed| {
        let mut val = *seed;
        for mapping in &mappings {
            val = convert(val, &mapping);
        }
        val
    }).min().unwrap())
}

// TODO: Rewrite to proper ranges instead of brute-force
pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, mappings) = parse(input);

    let mut min = u64::MAX;
    for i in 0..seeds.len() / 2 {
        let start = seeds[i * 2];
        let count = seeds[i * 2 + 1];

        for seed in start..start + count {
            let mut val = seed;
            for mapping in &mappings {
                val = convert(val, &mapping);
            }
            if val < min {
                min = val;
            }
        }
    }
    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
