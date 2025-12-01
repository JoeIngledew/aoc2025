advent_of_code::solution!(1);

enum Direction {
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Direction::Left => "LEFT",
            Direction::Right => "RIGHT",
        };
        write!(f, "{}", str)
    }
}

struct Instruct {
    direction: Direction,
    num: u64,
}

impl std::fmt::Display for Instruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", &self.direction, &self.num)
    }
}

fn to_instruction(s: &str) -> Instruct {
    let (a, b) = s.split_at(1);
    let num = b.parse::<u64>().unwrap();
    let direction = if a == "L".to_string() {
        Direction::Left
    } else {
        Direction::Right
    };
    Instruct { direction, num }
}

fn read_input(input: &str) -> Vec<Instruct> {
    input.split_ascii_whitespace().map(to_instruction).collect()
}

// dumbest possible solve but whatever
fn run_an_instruction(prev: (u64, u64), instruction: &Instruct, count_passing: bool) -> (u64, u64) {
    let (prev_num, prev_zero) = prev;
    let mut counter = instruction.num;
    let mut curr = prev_num;
    let mut zero_count = prev_zero;
    while counter > 0 {
        counter -= 1;
        match (&instruction.direction, curr) {
            (Direction::Left, 0) => curr = 99u64,
            (Direction::Right, 99) => curr = 0u64,
            (Direction::Left, _) => curr -= 1,
            (Direction::Right, _) => curr += 1,
        };
        if curr == 0 && (count_passing || counter == 0) {
            zero_count += 1;
        }
    }
    (curr, zero_count)
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = read_input(input);
    let res = instructions.iter().fold((50u64, 0u64), |acc, curr| {
        run_an_instruction(acc, curr, false)
    });
    let (_, zeroes) = res;
    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = read_input(input);
    let res = instructions.iter().fold((50u64, 0u64), |acc, curr| {
        run_an_instruction(acc, curr, true)
    });
    let (_, zeroes) = res;
    Some(zeroes)
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
        assert_eq!(result, Some(6));
    }
}
