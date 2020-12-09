use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_from_file("./src/input.txt");
    let part_one_num = part_one(&input, 25).unwrap();
    println!("Part one solution: {}", part_one_num);
    let part_two_num = part_two(&input, 25).unwrap();
    println!("Part two solution: {}", part_two_num);
}

fn read_from_file(path: &str) -> Vec<i64> {
    let input = read_to_string(path).expect("cannot parse file");

    input
        .trim()
        .lines()
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
}

fn has_valid_sum(prev_nums: &[i64], target: i64) -> bool {
    let mut visited = HashSet::<i64>::new();
    for &num in prev_nums.iter() {
        let need = target - num;
        if visited.contains(&need) {
            return true;
        }
        visited.insert(num);
    }

    false
}

fn part_one(input: &Vec<i64>, preamble_size: usize) -> Option<i64> {
    for window in input.windows(preamble_size + 1) {
        let target = *window.last().unwrap();
        let prev_nums = &window[..preamble_size];
        if !has_valid_sum(prev_nums, target) {
            return Some(target);
        }
    }

    None
}

fn part_two(input: &Vec<i64>, preamble_size: usize) -> Option<i64> {
    let part_one_sum = part_one(&input, preamble_size).unwrap();
    for window_size in 2..=input.len() {
        for window in input.windows(window_size) {
            let curr_sum: i64 = window.iter().sum();
            if curr_sum == part_one_sum {
                return Some(
                    window.iter().min().unwrap() + window.iter().max().unwrap()
                );
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{read_from_file, part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = read_from_file("./src/test_input.txt");
        assert_eq!(part_one(&input, 5).unwrap(), 127);
    }

    #[test]
    fn test_part_two() {
        let input = read_from_file("./src/test_input.txt");
        assert_eq!(part_two(&input, 5).unwrap(), 62);
    }
}
