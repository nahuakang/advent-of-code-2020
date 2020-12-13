use std::fs::read_to_string;

fn main() {
    let (depart_time, bus_table) = load_table("./src/input.txt");
    let part_one_answer = part_one(depart_time, &bus_table);
    println!("Part one answer: {}", part_one_answer);
    let part_two_answer = part_two(&bus_table);
    println!("Part two answer: {}", part_two_answer);
}

fn load_table(file_path: &str) -> (i64, Vec<(i64, i64)>) {
    let input = read_to_string(file_path).expect("cannot read to string");
    let mut input = input.trim().split("\n");
    let (depart_time, bus_table) = (input.next().unwrap(), input.next().unwrap());

    let bus_table: Vec<_> = bus_table
        .split(",")
        .enumerate()
        .filter(|(_, t)| t != &"x")
        .map(|(idx, time_str)| (idx as i64, time_str.parse::<i64>().unwrap()))
        .collect();
    
    (depart_time.parse::<i64>().unwrap(), bus_table)
}

fn part_one(depart_time: i64, bus_table: &Vec<(i64, i64)>) -> i64 {    
    let (earliest_bus, waiting_time) = find_earliest_bus(depart_time, bus_table);
    earliest_bus * waiting_time
}

fn find_earliest_bus(depart_time: i64, bus_table: &Vec<(i64, i64)>) -> (i64, i64) {    
    bus_table
        .iter()
        .map(|bus| {
            let waiting_time = bus.1 - depart_time.rem_euclid(bus.1);
            (bus.1, waiting_time)
        })
        .min_by_key(|bus| bus.1)
        .unwrap()
}

fn part_two(bus_table: &Vec<(i64, i64)>) -> i64 {
    let modulii = bus_table.iter().map(|&(_, bus)| bus).collect::<Vec<_>>();
    let residues = bus_table.iter().map(|&(idx ,bus)| (bus - idx)).collect::<Vec<_>>();
    chinese_remainder_theorem(&residues, &modulii).unwrap()
}

// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder_theorem(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

#[cfg(test)]
mod tests {
    use super::{load_table, part_one, part_two};

    #[test]
    fn test_part_one() {
        let (depart_time, bus_table) = load_table("./src/test_input.txt");
        assert_eq!(part_one(depart_time, &bus_table), 295);
    }

    #[test]
    fn test_part_two() {
        let (_, bus_table) = load_table("./src/test_input.txt");
        assert_eq!(part_two(&bus_table), 1068781);
    }
}
