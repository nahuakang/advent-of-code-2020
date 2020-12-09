use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let input = read_from_file("./src/input.txt");
    let part_one_num = find_invalid_number(&input, 25).unwrap();
    println!("Part one solution: {}", part_one_num);
    let part_two_num = find_weakness(&input, 25).unwrap();
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

fn find_invalid_number(input: &Vec<i64>, preamble_size: usize) -> Option<i64> {
    input
        .windows(preamble_size + 1)
        .find(|window| {
            window[..preamble_size]
                .iter()
                .combinations(2)
                .all(|combo| window[preamble_size] != combo.iter().copied().sum())
        })
        .map(|window| window[preamble_size])
}

fn find_weakness(input: &Vec<i64>, preamble_size: usize) -> Option<i64> {
    let invalid_number = find_invalid_number(input, preamble_size).unwrap();
    for size in 2..input.len() {
        if let Some(window) = input
            .windows(size)
            .find(|&window| invalid_number == window.iter().sum())
        {
            return Some(window.iter().max().unwrap() + window.iter().min().unwrap());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{find_invalid_number, find_weakness, read_from_file};

    #[test]
    fn test_part_one() {
        let input = read_from_file("./src/test_input.txt");
        assert_eq!(find_invalid_number(&input, 5).unwrap(), 127);
    }

    #[test]
    fn test_part_two() {
        let input = read_from_file("./src/test_input.txt");
        assert_eq!(find_weakness(&input, 5).unwrap(), 62);
    }
}
