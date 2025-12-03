advent_of_code::solution!(2);

// some slightly wobbly unwraps to be tidied up / handled properly
fn parse_range(s: &str) -> Vec<u64> {
    let (start, end) = s.split_once("-").unwrap();
    let start_n = start.parse::<u64>().unwrap();
    let end_n = end.parse::<u64>().unwrap();
    let mut res = Vec::new();
    for i in start_n..(end_n + 1) {
        res.push(i);
    }
    res
}

fn is_id_invalid(id_num: &&u64) -> bool {
    let id = id_num.to_string();
    let length = id.len();
    if length % 2 == 1 {
        false
    } else {
        let (a, b) = id.split_at(length / 2);
        a == b
    }
}

// again, must be a better way to do this, possibly using
// factors of 10 and modulo or smth
// but alas I am dumb
fn is_id_invalid_pt2(id: &&u64) -> bool {
    let mut invalid = false;
    let id_str = id.to_string();
    let len = id_str.len();
    let max_slice_size = len / 2;
    let mut current_slice_size: usize = 1;
    while !invalid && current_slice_size <= max_slice_size {
        // ignore if not evenly divisible
        if len.is_multiple_of(current_slice_size) {
            let (head, tail) = id_str.split_at(current_slice_size);
            let repeats = (len / current_slice_size) - 1;
            let cmp = vec![head; repeats].concat();
            invalid = cmp == *tail
        }
        current_slice_size += 1;
    }
    invalid
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input
        .split(",")
        .map(parse_range)
        .collect::<Vec<Vec<u64>>>()
        .concat();
    let res = ranges.iter().filter(is_id_invalid).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input
        .split(",")
        .map(parse_range)
        .collect::<Vec<Vec<u64>>>()
        .concat();
    let res = ranges.iter().filter(is_id_invalid_pt2).sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
