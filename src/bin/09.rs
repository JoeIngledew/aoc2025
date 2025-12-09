advent_of_code::solution!(9);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(",").unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        Point { x, y }
    }
}

fn manhattan(p1: &Point, p2: &Point) -> usize {
    let dist_x = p1.x.abs_diff(p2.x);
    let dist_y = p1.y.abs_diff(p2.y);
    dist_x + dist_y
}

fn area(p1: &Point, p2: &Point) -> usize {
    let dist_x = p1.x.abs_diff(p2.x);
    let dist_y = p1.y.abs_diff(p2.y);
    dist_x * dist_y
}

use itertools::*;

fn all_pairs<T>(xs: &[T]) -> Vec<(&T, &T)> {
    xs.iter().tuple_combinations().collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    let pairs: Vec<(&Point, &Point)> = all_pairs(&points);
    let (max_a, max_b) = pairs.iter().max_by(|(a1, a2), (b1, b2)| {
        let dist_a = manhattan(a1, a2);
        let dist_b = manhattan(b1, b2);
        dist_a.cmp(&dist_b)
    }).unwrap();
    dbg!(max_a);
    dbg!(max_b);
    let area = area(*max_a, *max_b);
    Some(area as u64)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
