use std::collections::HashMap;

advent_of_code::solution!(6);

enum Operator {
    Plus,
    Mult,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "*" => Operator::Mult,
            "+" => Operator::Plus,
            _ => panic!("Unrecognised operator"),
        }
    }
}

impl TryFrom<char> for Operator {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Operator::Mult),
            '+' => Ok(Operator::Plus),
            _ => Err("bad char"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut reversed = lines.rev();
    let operator_line: Vec<Operator> = reversed
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(Operator::from)
        .collect();
    let mut nums: HashMap<usize, Vec<u64>> = HashMap::new();
    for line in reversed {
        for (ix, n) in line
            .split_ascii_whitespace()
            .map(str::parse::<u64>)
            .enumerate()
        {
            let n = n.unwrap();
            nums.entry(ix).and_modify(|v| v.push(n)).or_insert(vec![n]);
        }
    }
    let result = operator_line
        .iter()
        .enumerate()
        .map(|(ix, op)| {
            let xs = nums.get(&ix).unwrap();
            match op {
                Operator::Plus => xs.iter().sum::<u64>(),
                Operator::Mult => xs.iter().product(),
            }
        })
        .sum();
    Some(result)
}

fn _pt2_2(_input: &str) {
    // let mut map: HashMap<usize, Vec<char>> = HashMap::new();
    // for l in input.lines() {
    //     for (ix, c) in l.chars().enumerate() {
    //         //map.entry(&ix).and_modify(|v| v)
    //         todo!();
    //     }
    // }
}

fn _pt2(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut reversed = lines.rev();
    let _op_line: Vec<(usize, char)> = reversed.next().unwrap().chars().rev().enumerate().collect();
    let original_order: Vec<&str> = reversed.rev().collect();
    let _split_strings: Vec<Vec<(usize, char)>> = original_order
        .iter()
        .map(|s| {
            let chars: Vec<(usize, char)> = s.chars().rev().enumerate().collect();
            chars
        })
        .collect();
    // for ((op_ix, op), ) in op_line.iter().zip(split_strings) {

    // }
    todo!()
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut reversed = lines.rev();
    let _operator_line: Vec<(usize, Operator)> = reversed
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(cix, c)| match Operator::try_from(c) {
            Ok(op) => Some((cix, op)),
            Err(_) => None,
        })
        .collect();

    let num_lines: Vec<&str> = reversed.rev().collect();
    // let char_indexed: Vec<(usize, usize, char)> = num_lines
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(y, l)| l.chars().enumerate().map(|(x, c)| (x, y, c))))
    //     .collect();
    // let mut nums: Vec<(usize, Vec<char>)> =

    let mut nums: HashMap<usize, Vec<&str>> = HashMap::new();
    for line in &num_lines {
        for (ix, n) in line.split_ascii_whitespace().enumerate() {
            nums.entry(ix).and_modify(|v| v.push(n)).or_insert(vec![n]);
        }
    }
    // let result = operator_line.iter().enumerate().map(|(ix, op)| {
    //     let strs = nums.get(&ix).unwrap();
    //     let digits = strs.iter().enumerate().map()
    // })

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
