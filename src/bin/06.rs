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

fn pt2_again(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let lines_count = lines.len();
    let column_count = lines[0].len();
    let op_line = lines[lines_count - 1];
    let other_lines: Vec<String> = lines
        .iter()
        .cloned()
        .take(lines_count - 1)
        .map(str::to_string)
        .collect();

    let mut result: u64 = 0;
    let mut number_scratchpad: Vec<u64> = Vec::new();
    for x in 0..column_count {
        let ix = column_count - (x + 1);
        let op = op_line.chars().nth(ix).unwrap();
        let nums: String = other_lines
            .iter()
            .map(|f| f.chars().nth(ix).unwrap())
            .collect();
        let trimmed = nums.trim();
        if let Ok(n) = trimmed.parse::<u64>() {
            number_scratchpad.push(n);
        }
        match Operator::try_from(op) {
            Ok(Operator::Mult) => {
                let product: u64 = number_scratchpad.iter().product();
                result += product;
                number_scratchpad.clear();
            }
            Ok(Operator::Plus) => {
                let sum: u64 = number_scratchpad.iter().sum();
                result += sum;
                number_scratchpad.clear();
            }
            Err(_) => (),
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    pt2_again(input)
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
        assert_eq!(result, Some(3263827));
    }
}
