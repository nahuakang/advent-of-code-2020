use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let mut adapters = load_adapters("./src/input.txt");
    let part_one_answer = part_one(&mut adapters);
    println!("Part one answer: {}", part_one_answer);
    println!("adapters: {:?}", adapters);
    let part_two_answer = part_two(&adapters);
    println!("Part two answer: {}", part_two_answer);
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
    println!("adapters: {:?}", adapters);
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

fn count_routes(adapters: &Vec<usize>) -> HashMap<isize, usize> {
    let mut routes = HashMap::<isize, usize>::new();
    
    routes.insert(0, 1);
    
    for &adapter in adapters[1..].iter() {
        let mut count = 0;
        println!("adapter is {}", adapter);
        for i in (adapter as isize - 3isize)..(adapter as isize) {
            let num = match routes.get(&i) {
                Some(&num) => num,
                None => 0,
            };
            println!("count is {}, num is {}", count, num);
            count += num;
        }
        routes.insert(adapter as isize, count);
    }
    println!("routes: {:?}", routes);

    routes
}

fn part_two(adapters: &Vec<usize>) -> usize {
    *count_routes(adapters).values().max().unwrap()
}

#[cfg(test)]
mod tests{
    use super::{load_adapters, part_one, part_two};

    #[test]
    fn test_part_one() {
        let mut input = load_adapters("./src/test_input.txt");
        assert_eq!(part_one(&mut input), 35);
    }

    #[test]
    fn test_part_two() {
        let input = load_adapters("./src/test_input.txt");
        assert_eq!(part_two(&input), 8);
    }
}
