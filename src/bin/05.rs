use std::collections::HashMap;

advent_of_code::solution!(5);

use std::ops::Range;

use range_overlap::{RangeOverlap, classify_any};
struct Input2 {
    ranges: Vec<Range<u64>>,
    available: Vec<u64>,
}

impl From<&str> for Input2 {
    fn from(value: &str) -> Self {
        let mut now_logging_available = false;
        let mut avail: Vec<u64> = Vec::new();
        let mut valid: Vec<Range<u64>> = Vec::new();
        let copy_value_lines = value.lines();
        for s in copy_value_lines {
            if now_logging_available {
                let a = s.parse::<u64>().unwrap();
                avail.push(a);
            } else if s.is_empty() {
                now_logging_available = true;
            } else {
                let (x, y) = s.split_once("-").unwrap();
                let x = x.parse::<u64>().unwrap();
                let y = y.parse::<u64>().unwrap();
                valid.push(x..(y + 1));
            }
        }
        Self {
            ranges: valid,
            available: avail,
        }
    }
}

fn _try_join_ranges(
    r1: &Range<u64>,
    r2: &Range<u64>,
) -> Result<Range<u64>, (Range<u64>, Range<u64>)> {
    let is_r1_start_in_r2 = r1.start < r2.end && r1.start > r2.start;
    let is_r2_start_in_r1 = r2.start < r1.end && r2.start > r1.start;
    let is_r1_end_in_r2 = r1.end < r2.end && r1.end > r2.start;
    let is_r2_end_in_r1 = r2.end < r1.end && r2.end > r1.start;
    match (
        is_r1_start_in_r2,
        is_r1_end_in_r2,
        is_r2_start_in_r1,
        is_r2_end_in_r1,
    ) {
        (true, true, _, _) => Ok(r2.clone()),
        (_, _, true, true) => Ok(r1.clone()),
        (false, false, false, false) => Err((r1.clone(), r2.clone())),
        (true, false, _, _) => Ok(r2.start..r1.end),
        (_, _, true, false) => Ok(r1.start..r2.end),
        (false, true, false, true) => panic!(),
        (false, true, false, false) => panic!(),
        (false, false, false, true) => panic!(), // all 3 are impossible I think
    }
}

fn _reduce_ranges(acc: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let next: Vec<Range<u64>> = acc
        .chunks(2)
        .flat_map(|c| {
            if c.len() > 1 {
                let res: Result<Range<u64>, (Range<u64>, Range<u64>)> =
                    _try_join_ranges(&c[0], &c[1]);
                match res {
                    Ok(r) => vec![r],
                    Err((r1, r2)) => vec![r1, r2],
                }
            } else {
                let res: Vec<Range<u64>> = c.to_vec();
                res
            }
        })
        .collect();
    let next2: Vec<Range<u64>> = next
        .rchunks(2)
        .flat_map(|c| {
            if c.len() > 1 {
                let res: Result<Range<u64>, (Range<u64>, Range<u64>)> =
                    _try_join_ranges(&c[0], &c[1]);
                match res {
                    Ok(r) => vec![r],
                    Err((r1, r2)) => vec![r1, r2],
                }
            } else {
                let res: Vec<Range<u64>> = c.to_vec();
                res
            }
        })
        .collect();
    if next2.len() == acc.len() {
        next2
    } else {
        _reduce_ranges(next2)
    }
}

fn _fold_reduce_ranges(acc: Vec<Range<u64>>, curr: &Range<u64>) -> Vec<Range<u64>> {
    let mut next: Vec<Range<u64>> = Vec::new();
    let mut init = curr.start..curr.end;
    let start_acc_len = &acc.len();
    for r in acc {
        match classify_any(
            Some(r.start),
            Some(r.end),
            Some(init.start),
            Some(init.end),
            false,
        ) {
            RangeOverlap::AEndsInB => init = r.start..init.end,
            RangeOverlap::AStartsInB => init = init.start..r.end,
            RangeOverlap::AEqualsB => (),
            RangeOverlap::AInsideB => (),
            RangeOverlap::None => {
                next.push(r);
            }
            RangeOverlap::AContainsB => {
                next.push(r);
            }
        };
    }
    next.push(init);
    if &next.len() == start_acc_len {
        next
    } else {
        _fold_reduce_ranges(next, curr)
    }
}

fn _get_overlaps(
    ranges: &HashMap<u64, u64>,
    start: u64,
    end: u64,
    _existing: Vec<u64>,
) -> Vec<(Vec<u64>, (u64, u64))> {
    let overlapping: Vec<(u64, (u64, u64))> = ranges
        .iter()
        .filter_map(|(a, b)| {
            let overlap = classify_any(Some(start), Some(end), Some(*a), Some(*b), false);
            match overlap {
                RangeOverlap::AContainsB => Some((*a, (start, end))),
                RangeOverlap::AInsideB => None,
                RangeOverlap::AEndsInB => Some((*a, (start, *b))),
                RangeOverlap::AStartsInB => Some((*a, (*a, end))),
                RangeOverlap::AEqualsB => None,
                RangeOverlap::None => None,
            }
        })
        .collect();
    if overlapping.is_empty() {
        todo!()
    }
    todo!()
}

fn collapse_ranges(ranges: HashMap<u64, u64>, new_range: (u64, u64)) -> HashMap<u64, u64> {
    let mut scratch = ranges.clone();
    let next_new = collapse_ranges_once(&mut scratch, new_range);
    if next_new == new_range {
        scratch
    } else {
        collapse_ranges(scratch, next_new)
    }
}

fn collapse_ranges_once(ranges: &mut HashMap<u64, u64>, new_range: (u64, u64)) -> (u64, u64) {
    let mut start = new_range.0;
    let mut end = new_range.1;
    let mut to_remove: Vec<u64> = Vec::new();
    for (a, b) in ranges.iter() {
        let overlap = classify_any(Some(start), Some(end), Some(*a), Some(*b), false);
        match overlap {
            RangeOverlap::AContainsB => {
                to_remove.push(*a);
            }
            RangeOverlap::AInsideB => (),
            RangeOverlap::AEndsInB => {
                to_remove.push(*a);
                end = *b;
            }
            RangeOverlap::AStartsInB => {
                to_remove.push(*a);
                start = *a;
            }
            RangeOverlap::AEqualsB => (),
            RangeOverlap::None => (),
        }
    }
    ranges.insert(start, end);
    for r in to_remove {
        ranges.remove(&r);
    }
    (start, end)
}

// fn pt2(input: &str) {
//     let mut ranges: HashMap<u64, u64> = HashMap::new();
//     let mut lines = input.lines();
//     while let Some(l) = lines.next() && !l.is_empty() {
//         let (start, end) = l.split_once("-").unwrap();
//         let start = start.parse::<u64>().unwrap();
//         let end = end.parse::<u64>().unwrap() + 1;
//         let init = vec![(start, end)];
//         let overlapping2 = ranges.iter().fold(init, |acc, curr| {

//         })
//         let overlapping: Vec<(u64, (u64, u64))> = ranges.iter().filter_map(|(a, b)| {
//             let overlap = classify_any(Some(start), Some(end), Some(*a), Some(*b), false);
//             match overlap {
//                 RangeOverlap::AContainsB => Some((*a, (start, end))),
//                 RangeOverlap::AInsideB => None,
//                 RangeOverlap::AEndsInB => Some((*a, (start, *b))),
//                 RangeOverlap::AStartsInB => Some((*a, (*a, end))),
//                 RangeOverlap::AEqualsB => None,
//                 RangeOverlap::None => None,
//             }
//         }).collect();
//         if overlapping.len() == 0 {

//         } else {}
//         for (r, (s, e)) in overlapping {
//             ranges.remove(&r);
//             ranges.insert(s, e);
//         }
//     }
// }

fn pt2(input: &str) -> u64 {
    let mut ranges: HashMap<u64, u64> = HashMap::new();
    let mut lines = input.lines();
    while let Some(l) = lines.next()
        && !l.is_empty()
    {
        let (start, end) = l.split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap() + 1;
        ranges = collapse_ranges(ranges, (start, end));
    }
    let xs = ranges.iter().map(|(s, e)| *e - *s);
    let res: u64 = xs.sum();
    res
}

impl Input2 {
    fn count_fresh(&self) -> u64 {
        self.available
            .iter()
            .filter(|c| self.ranges.iter().any(|r| r.contains(*c)))
            .count() as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = Input2::from(input);
    Some(input.count_fresh())
}

pub fn part_two(input: &str) -> Option<u64> {
    // let input = Input2::from(input);
    // let mut start_ranges = input.ranges.clone();
    // dbg!(&start_ranges.len());
    // start_ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    // let ranges = reduce_ranges(input.ranges.clone());
    // dbg!(&ranges.len());
    // let combined_length: u64 = ranges.iter().map(|r| r.end - r.start).sum();
    // Some(combined_length)
    let res = pt2(input);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
