advent_of_code::solution!(4);

fn parse(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input.lines().map(|line| {
        let mut parts = line.split(": ").nth(1).unwrap().split(" | ");
        let winning_numbers: Vec<u32> = parts.next().unwrap().split_whitespace().filter_map(|n| n.parse().ok()).collect();
        let our_numbers: Vec<u32> = parts.next().unwrap().split_whitespace().filter_map(|n| n.parse().ok()).collect();
        (winning_numbers, our_numbers)
    }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).iter().map(|(win, our)| {
        u32::pow(2, win.iter().filter(|w| our.contains(w)).count() as u32) / 2
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse(input);
    let mut n_cards = vec![1; cards.len()];
    for (i, (win, our)) in cards.iter().enumerate() {
        let index = cards.len().min(i + 1 + win.iter().filter(|w| our.contains(w)).count());
        for j in i + 1..index {
            n_cards[j] += n_cards[i]
        }
    }
    Some(n_cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
