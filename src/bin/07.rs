use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Item {
    Split = 0,
    Start = 1,
    Empty = 2,
    Laser = 3,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            'S' => Item::Start,
            '^' => Item::Split,
            '|' => Item::Laser, // not possible from input but whatever
            _ => Item::Empty,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Point {
    col: usize,
    line: usize,
}

impl Point {
    fn new(col: usize, line: usize) -> Self {
        Self { col, line }
    }
}

fn parse(input: &str) -> (HashMap<Point, Item>, Point) {
    let mut map: HashMap<Point, Item> = HashMap::new();
    let mut start = Point::new(0, 0);
    for (line, s) in input.lines().enumerate() {
        for (col, c) in s.chars().enumerate() {
            let item = Item::from(c);
            map.insert(Point::new(col, line), item);
            if item == Item::Start {
                start = Point::new(col, line);
            }
        }
    }
    (map, start)
}

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
