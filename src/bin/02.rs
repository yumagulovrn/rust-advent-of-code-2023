use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut res: u32 = 0;
    let bag: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for (n, line) in input.lines().enumerate() {
        let mut possible = true;
        for game in line.split(": ").last().unwrap().split("; ") {
            for action in game.split(", ") {
                let action: Vec<&str> = action.split_whitespace().collect();
                let num = action.first().unwrap().parse::<u32>().unwrap();
                let limit = bag.get(*action.last().unwrap()).unwrap();
                if num > *limit {
                    possible = false;
                    break;
                }
            }
            if !possible {
                break;
            };
        }
        if possible {
            res += n as u32 + 1
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res: u32 = 0;

    for line in input.lines() {
        let mut bag: HashMap<&str, u32> = HashMap::new();
        for game in line.split(": ").last().unwrap().split("; ") {
            for action in game.split(", ") {
                let action: Vec<&str> = action.split_whitespace().collect();
                let num = action.first().unwrap().parse::<u32>().unwrap();
                let color = *action.last().unwrap();
                let limit = bag.get(color).unwrap_or(&0);
                if num > *limit {
                    bag.insert(color, num);
                }
            }
        }
        res += bag.values().product::<u32>()
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
