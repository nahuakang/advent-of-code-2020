use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_file("./src/input.txt");

    let result = find_two_num_to_sum(&input, 2020);
    match result {
        Ok(i) => println!("The multiplication is {}", i),
        Err(e) => println!("{}", e),
    }
}

fn read_file(path_to_file: &str) -> Vec<i32> {
    let mut result = vec![];
    let file_handle = File::open(path_to_file).expect("cannot open file");
    let file = BufReader::new(file_handle);

    for line in file.lines() {
        let line = line.expect("unable to read line");
        let num: i32 = line.trim().parse().expect("cannot parse invalid string number");
        result.push(num);
    }

    result
}

fn find_two_num_to_sum(input: &Vec<i32>, sum: i32) -> Result<i32, &'static str> {
    let mut has_seen: HashMap<i32, bool> = HashMap::new();

    for &i in input {
        let need = sum - i;
        if has_seen.contains_key(&need) {
            return Ok(need * i);
        }
        has_seen.insert(i, true);
    }

    Err("Cannot find two numbers summing to 2020")
}
