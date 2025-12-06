use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn around(&self) -> Vec<Self> {
        let mut points: Vec<Self> = Vec::new();
        points.push(Self {
            x: self.x + 1,
            y: self.y,
        });
        points.push(Self {
            x: self.x + 1,
            y: self.y + 1,
        });
        points.push(Self {
            x: self.x,
            y: self.y + 1,
        });
        if self.x != 0 && self.y != 0 {
            points.push(Self {
                x: self.x - 1,
                y: self.y - 1,
            });
        }
        if self.x != 0 {
            points.push(Self {
                x: self.x - 1,
                y: self.y,
            });
            points.push(Self {
                x: self.x - 1,
                y: self.y + 1,
            });
        }
        if self.y != 0 {
            points.push(Self {
                x: self.x,
                y: self.y - 1,
            });
            points.push(Self {
                x: self.x + 1,
                y: self.y - 1,
            });
        }
        points
    }
}

fn parse_input(input: &str) -> HashMap<Point, bool> {
    let lines = input.lines().clone();
    let mut res: HashMap<Point, bool> = HashMap::new();
    for (y, line) in lines.enumerate() {
        let chars = line.chars();
        for (x, c) in chars.enumerate() {
            let point = Point { x, y };
            let is_paper = c == '@';
            res.insert(point, is_paper);
        }
    }
    res
}

// again very naiive solutions - still performing "well enough"
pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let mut valid_points: Vec<Point> = Vec::new();
    for (k, v) in &map {
        // only look if it's a paper square
        if *v {
            let around = k.around();
            let around_paper = around
                .iter()
                .filter(|p| if let Some(a) = map.get(p) { *a } else { false });
            if around_paper.count() < 4 {
                valid_points.push(Point::new(k.x, k.y));
            }
        }
    }
    Some(valid_points.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = parse_input(input);
    let mut can_remove = true;
    let mut count_removed: u64 = 0;
    while can_remove {
        let mut valid_points: Vec<Point> = Vec::new();
        for (k, v) in &map {
            // only look if it's a paper square
            if *v {
                let around = k.around();
                let around_paper = around
                    .iter()
                    .filter(|p| if let Some(a) = map.get(p) { *a } else { false });
                if around_paper.count() < 4 {
                    valid_points.push(Point::new(k.x, k.y));
                }
            }
        }
        let len = valid_points.len();

        if len != 0 {
            count_removed += len as u64;
            for p in valid_points {
                map.entry(p).and_modify(|e| *e = false);
            }
        } else {
            can_remove = false;
        }
    }
    Some(count_removed)
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
        assert_eq!(result, Some(43));
    }
}
