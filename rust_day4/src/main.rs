use std::fs::read_to_string;
use std::ops::RangeInclusive;
use regex::Regex;

fn main() {
    let valid_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports_input = read_to_string("./src/input.txt").expect("cannot load file");
    let passports: Vec<&str> = passports_input.split("\n\n").collect();

    println!("Last item in passports is: {:?}", passports.last());

    let part_one_valid_count: i32 = passports
        .iter()
        .map(|&passport| has_required_fields(passport, &valid_fields) as i32)
        .sum();
    println!("Part 1: Valid count is {}.", part_one_valid_count);

    let part_two_valid_count: i32 = passports
        .iter()
        .filter(|&passport| has_required_fields(passport, &valid_fields))
        .map(|&passport| has_valid_fields(passport) as i32)
        .sum();
    println!("Part 2: Valid count is {}.", part_two_valid_count);
}

fn has_required_fields(passport: &str, required_fields: &Vec<&str>) -> bool {
    required_fields.iter().all(|&required| passport.contains(required))
}

fn has_valid_fields(passport: &str) -> bool {
    let fields_separator = Regex::new(r"\s\n|\s|\n").unwrap();

    let mut fields: Vec<&str> = fields_separator.split(passport).collect();

    // Drop the final nextline to avoid
    println!("fields are: {:?}", fields);
    if fields.ends_with(&[""]) {
        fields.pop();
        println!("fields cleaned are: {:?}", fields);
    }
    
    fields.iter().all(|field| is_valid_field(field))
}

fn is_valid_field(field: &str) -> bool {
    println!("Field is currently: {}", field);
    let field_extractor = Regex::new(r":").unwrap();
    let splitted_field: Vec<&str> = field_extractor.split(field).collect();
    let (key, value) = (splitted_field[0], splitted_field[1]);
    
    match key {
        "byr" => is_in_range(value, 1920..=2002),
        "iyr" => is_in_range(value, 2010..=2020),
        "eyr" => is_in_range(value, 2020..=2030),
        "hgt" => is_in_range_suffix(value, 150..=193, "cm") || is_in_range_suffix(value, 59..=76, "in"),
        // https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_hexdigit
        "hcl" => value.strip_prefix("#").map_or(false, |color| color.chars().all(|c| c.is_ascii_hexdigit())),
        // https://doc.rust-lang.org/std/macro.matches.html
        "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => false,
    }
}

fn is_in_range(value: &str, range: RangeInclusive<usize>) -> bool {
    // https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or
    value.parse().map_or(false, |num: usize| range.contains(&num))
}

fn is_in_range_suffix(value: &str, range: RangeInclusive<usize>, suffix: &str) -> bool {
    // https://doc.rust-lang.org/std/string/struct.String.html#method.strip_suffix
    value.strip_suffix(suffix).map_or(false, |num| is_in_range(num, range))
}
