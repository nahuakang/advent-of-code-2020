use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let mut adapters = load_adapters("./src/input.txt");
    let part_one_answer = part_one(&mut adapters);
    println!("Part one answer: {}", part_one_answer);
}

fn load_adapters(path: &str) -> Vec<usize> {
    let input = read_to_string(path).expect("cannot parse file");

    input
        .trim()
        .lines()
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn chain_and_find_diffs(adapters: &mut Vec<usize>) -> HashMap<usize, usize> {
    let mut diff_counter = HashMap::new();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.insert(adapters.len(), adapters.last().unwrap() + 3);
    for pair in adapters.windows(2) {
        let diff = pair[1] - pair[0];
        let counter = diff_counter.entry(diff).or_insert(0);
        *counter += 1;
    }

    diff_counter
}

fn part_one(adapters: &mut Vec<usize>) -> usize {
    let diff_counter = chain_and_find_diffs(adapters);
    println!("diff counter: {:?}", diff_counter);
    diff_counter[&1] * diff_counter[&3]
}

#[cfg(test)]
mod tests{
    use super::{load_adapters, part_one};

    #[test]
    fn test_part_one() {
        let mut input = load_adapters("./src/test_input.txt");
        assert_eq!(part_one(&mut input), 35);
    }
}
