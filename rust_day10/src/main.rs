use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let adapters = load_adapters("./src/input.txt");
    let part_one_answer = part_one(&adapters);
    println!("Part one answer: {}", part_one_answer);
    let part_two_answer = part_two(&adapters);
    println!("Part two answer: {}", part_two_answer);
}

fn load_adapters(path: &str) -> Vec<usize> {
    let input = read_to_string(path).expect("cannot parse file");

    let mut adapters: Vec<usize> = input
        .trim()
        .lines()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    
    adapters.sort();
    adapters.insert(0, 0);
    adapters.insert(adapters.len(), adapters.last().unwrap() + 3);

    adapters
}

fn chain_and_find_diffs(adapters: &mut Vec<usize>) -> HashMap<usize, usize> {
    let mut diff_counter = HashMap::new();

    adapters.windows(2).for_each(|pair| {
        let diff = pair[1] - pair[0];
        let counter = diff_counter.entry(diff).or_insert(0);
        *counter += 1;
    });

    diff_counter
}

fn part_one(adapters: &Vec<usize>) -> usize {
    let mut adapters: Vec<usize> = adapters.iter().copied().collect();
    let diff_counter = chain_and_find_diffs(&mut adapters);
    
    diff_counter[&1] * diff_counter[&3]
}

fn count_routes(adapters: &Vec<usize>) -> HashMap<isize, usize> {
    let mut routes = HashMap::<isize, usize>::new();
    routes.insert(adapters[0] as isize, 1);
    
    // Each joltage route is equal to the sum of the number of routes to the previous three joltages.
    // For joltages that are not present in the list of adaptors, provide 0 as value.
    for &adapter in adapters[1..].iter() {
        let mut count = 0;
        for i in (adapter as isize - 3isize)..(adapter as isize) {
            let num = match routes.get(&i) {
                Some(&num) => num,
                None => 0,
            };
            count += num;
        }
        routes.insert(adapter as isize, count);
    }

    routes
}

fn part_two(adapters: &Vec<usize>) -> usize {
    *count_routes(&adapters).values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{load_adapters, part_one, part_two};

    #[test]
    fn test_part_one() {
        let mut input = load_adapters("./src/test_input.txt");
        assert_eq!(part_one(&mut input), 35);
        let mut input2 = load_adapters("./src/test_input2.txt");
        assert_eq!(part_one(&mut input2), 220);
    }

    #[test]
    fn test_part_two() {
        let input = load_adapters("./src/test_input.txt");
        assert_eq!(part_two(&input), 8);
        let input2 = load_adapters("./src/test_input2.txt");
        assert_eq!(part_two(&input2), 19208);
    }
}
