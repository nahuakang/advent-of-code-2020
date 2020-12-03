use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_file("./src/input.txt");

    let result_of_two = find_two_nums_to_sum(&input, 2020);
    match result_of_two {
        Ok(i) => println!("The multiplication is {}", i),
        Err(e) => println!("{}", e),
    }

    let result_of_three = find_three_nums_to_sum(&input, 2020);
    match result_of_three {
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

fn find_two_nums_to_sum(input: &Vec<i32>, sum: i32) -> Result<i32, &'static str> {
    let mut has_seen: HashMap<i32, bool> = HashMap::new();

    for &num in input {
        let need = sum - num;
        if has_seen.contains_key(&need) {
            return Ok(need * num);
        }
        has_seen.insert(num, true);
    }

    Err("Cannot find two numbers summing to 2020")
}

fn find_three_nums_to_sum(input: &Vec<i32>, sum: i32) -> Result<i32, &'static str> {
    for (idx, num) in input.iter().enumerate() {
        let need = sum - num;
        let result = find_two_nums_to_sum(
            &input[idx..].to_vec(),
            need,
        );
        match result {
            Ok(i) => return Ok(i * num),
            Err(_) => continue,
        }
    }

    Err("Cannot find three numbers summing to 2020")
}
