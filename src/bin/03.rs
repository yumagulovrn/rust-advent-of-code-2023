advent_of_code::solution!(3);

struct Point {
    x: i32,
    y: i32,
    is_star: bool,
}

struct Part {
    number: i32,
    x1: i32,
    x2: i32,
    y: i32,
}

impl Point {
    fn gear_ratio(&self, parts: &[Part]) -> i32 {
        if !self.is_star {
            return 0;
        }
        let mut ratio = 1;
        let mut adjacent_parts_count = 0;

        for part in parts {
            if part.is_adjacent_to_point(self) {
                adjacent_parts_count += 1;
                if adjacent_parts_count > 2 {
                    return 0;
                }
                ratio *= part.number;
            }
        }
        if adjacent_parts_count != 2 {
            return 0;
        }
        ratio
    }
}

impl Part {
    fn is_adjacent_to_point(&self, point: &Point) -> bool {
        (self.x1 - 1..=self.x2 + 1).contains(&point.x)
            && (self.y - 1..=self.y + 1).contains(&point.y)
    }

    fn is_adjacent_to_any_point(&self, points: &[Point]) -> bool {
        points.iter().any(|point| self.is_adjacent_to_point(point))
    }
}

fn parse(input: &str) -> (Vec<Part>, Vec<Point>) {
    let mut parts: Vec<Part> = vec![];
    let mut points: Vec<Point> = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut n: Option<i32> = None;
        let mut l: i32 = 0;
        for (x, &c) in line.as_bytes().iter().chain([b'.'].iter()).enumerate() {
            if c.is_ascii_digit() {
                l += 1;
                let digit = (c - b'0') as i32;
                n = n.map_or(Some(digit), |number| Some(number * 10 + digit));
            } else {
                if let Some(number) = n {
                    parts.push(Part {
                        number,
                        x1: x as i32 - l,
                        x2: x as i32 - 1,
                        y: y as i32,
                    })
                };
                n = None;
                l = 0;
                if c != b'.' {
                    points.push(Point {
                        x: x as i32,
                        y: y as i32,
                        is_star: c == b'*',
                    })
                }
            }
        }
    }
    (parts, points)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (parts, points) = parse(input);
    Some(
        parts
            .iter()
            .filter(|part| part.is_adjacent_to_any_point(&points))
            .map(|point| point.number)
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (parts, points) = parse(input);

    Some(
        points
            .iter()
            .map(|point| point.gear_ratio(&parts))
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
