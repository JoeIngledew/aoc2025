use std::collections::{HashMap, HashSet};

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

fn parse(input: &str) -> (HashMap<Point, Item>, Point, usize, usize) {
    let mut map: HashMap<Point, Item> = HashMap::new();
    let mut start = Point::new(0, 0);
    let mut max_y: usize = 0;
    let mut max_x: usize = 0;
    for (line, s) in input.lines().enumerate() {
        for (col, c) in s.chars().enumerate() {
            let item = Item::from(c);
            map.insert(Point::new(col, line), item);
            if item == Item::Start {
                start = Point::new(col, line);
            }
            max_x = col;
        }
        max_y = line;
    }
    (map, start, max_y, max_x)
}

fn solve_1(input: &str) -> u64 {
    let (points, start, max_y, max_x) = parse(input);
    let mut curr_laser_cols: HashSet<usize> = HashSet::new();
    let _ = &curr_laser_cols.insert(start.col);
    let mut count_splits: u64 = 0;
    for y in 1..max_y {
        for c in &curr_laser_cols.clone() {
            let p = Point::new(*c, y);
            if let Some(&Item::Split) = points.get(&p) {
                let _ = &curr_laser_cols.remove(&c);
                if *c != 0 {
                    let _ = &curr_laser_cols.insert(c-1);
                }
                if *c != max_x {
                    let _ = &curr_laser_cols.insert(c+1);
                }
                count_splits += 1;
            }
        }
    }
    count_splits
}

// pt1 mostly solved it but cba to adjust the function to fit both parts
fn solve_2(input: &str) -> u64 {
    let (points, start, max_y, max_x) = parse(input);
    let mut curr_laser_cols: HashSet<usize> = HashSet::new();
    let _ = &curr_laser_cols.insert(start.col);
    let mut timelines: u64 = 0;
    for y in 1..max_y {
        for c in &curr_laser_cols.clone() {
            let p = Point::new(*c, y);
            if let Some(&Item::Split) = points.get(&p) {
                let _ = &curr_laser_cols.remove(&c);
                if *c != 0 {
                    let _ = &curr_laser_cols.insert(c-1);
                    timelines += 1;
                }
                if *c != max_x {
                    let _ = &curr_laser_cols.insert(c+1);
                    timelines += 1;
                }
            }
        }
    }
    timelines
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = solve_1(input);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = solve_2(input);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
