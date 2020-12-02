use regex::Regex;

fn main() {
    // https://doc.rust-lang.org/std/macro.include_str.html
    let raw_input = include_str!("./input.txt");
    let re = Regex::new(r"[- :]+").unwrap();

    let sled_rental_pw_count = count_valid_sled_rental_passwords(raw_input, &re);
    println!("{}", sled_rental_pw_count);

    let toboggan_pw_count = count_valid_toboggan_passwords(raw_input, &re);
    println!("{}", toboggan_pw_count);
}

fn count_valid_sled_rental_passwords(input: &str, re: &Regex) -> i32 {
    let mut valid_count = 0; 
    for line in input.lines() {
        let password_info = re.split(line).collect::<Vec<&str>>();
        let cnt = password_info[3].matches(password_info[2]).count() as i32;
        if cnt >= password_info[0].parse::<i32>().unwrap() && cnt <= password_info[1].parse::<i32>().unwrap() {
            valid_count += 1;
        }
    }
    
    valid_count
}

fn count_valid_toboggan_passwords(input: &str, re: &Regex) -> i32 {
    let mut valid_count = 0;
    for line in input.lines() {
        let password_info = re.split(line).collect::<Vec<&str>>();
        let password = password_info[3];
        let letter = password_info[2];
        let first_pos = password_info[0].parse::<i32>().unwrap();
        let second_pos = password_info[1].parse::<i32>().unwrap();
        let first_pos_match = password.chars().nth(first_pos as usize - 1).unwrap().to_string() == letter;
        let second_pos_match = password.chars().nth(second_pos as usize - 1).unwrap().to_string() == letter;

        match (first_pos_match, second_pos_match) {
            (false, false) | (true, true) => {},
            (true, false) | (false, true) => valid_count += 1,
        }
    }
    
    valid_count
}
