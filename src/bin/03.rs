advent_of_code::solution!(3);

fn find_largest_num(cs: Vec<char>) -> u64 {
    let mut highest_a = '0';
    let mut highest_b = '0';
    let mut a_ix: usize = 0;

    let mut curr_ix: usize = 0;
    while highest_a != '9' && curr_ix < (cs.len() - 1) {
        if cs[curr_ix] > highest_a {
            highest_a = cs[curr_ix];
            a_ix = curr_ix;
        }
        curr_ix += 1;
    }
    curr_ix = a_ix + 1;
    while highest_b != '9' && curr_ix < cs.len() {
        if cs[curr_ix] > highest_b {
            highest_b = cs[curr_ix];
        }
        curr_ix += 1;
    }

    let final_str = format!("{}{}", highest_a, highest_b);
    final_str.parse::<u64>().unwrap()
}

fn find_largest_num_pt2(cs: Vec<char>) -> u64 {
    let mut digits_str = "".to_string();
    let mut start_ix = 0usize;
    for i in [11usize, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0] {
        let mut highest = '0';
        let mut highest_ix = 0usize;
        let mut curr_ix = start_ix;
        while highest != '9' && curr_ix < (cs.len() - i) {
            if cs[curr_ix] > highest {
                highest = cs[curr_ix];
                highest_ix = curr_ix;
            }
            curr_ix += 1;
        }
        digits_str = format!("{}{}", digits_str, highest);
        start_ix = highest_ix + 1;
    }
    digits_str.parse::<u64>().unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let res: u64 = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(find_largest_num)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res: u64 = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(find_largest_num_pt2)
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
