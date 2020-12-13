use std::fs::read_to_string;

fn main() {
    let (depart_time, bus_table) = load_table("./src/input.txt");
    let part_one_answer = part_one(&depart_time, &bus_table);
    println!("Part one answer: {}", part_one_answer);
}

fn load_table(file_path: &str) -> (String, String) {
    let input = read_to_string(file_path).expect("cannot read to string");
    let mut input = input.trim().split("\n");
    (input.next().unwrap().to_owned(), input.next().unwrap().to_owned())
}

fn find_earliest_bus(depart_time: &String, bus_table: &String) -> (usize, usize) {
    let depart_time: usize = depart_time.parse().unwrap();
    let bus_table: Vec<usize> = bus_table
        .split(",")
        .filter_map(|t| match t.parse::<usize>() {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect();
    
    bus_table
        .iter()
        .enumerate()
        .map(|(idx, &bus)| {
            let waiting_time = bus - depart_time.rem_euclid(bus);
            (bus_table[idx], waiting_time)
        })
        .min_by_key(|bus_and_time| bus_and_time.1)
        .unwrap()
}

fn part_one(depart_time: &String, bus_table: &String) -> usize {
    let (earliest_bus, waiting_time) = find_earliest_bus(depart_time, bus_table);
    earliest_bus * waiting_time
}

#[cfg(test)]
mod tests {
    use super::{load_table, part_one};

    #[test]
    fn test_part_one() {
        let (depart_time, bus_table) = load_table("./src/test_input.txt");
        assert_eq!(part_one(&depart_time, &bus_table), 295);
    }
}
