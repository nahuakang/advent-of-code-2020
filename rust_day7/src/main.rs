use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let luggage_rules = read_to_string("./src/input.txt").expect("cannot read from file");
    let luggage_rules: &str = luggage_rules.trim();

    let rules: HashMap<&str, Vec<&str>> = get_bag_rules(luggage_rules);

    let count = count_bags(&rules, "shiny gold");
    println!("Part 1 count: {}", count);
}

fn get_bag_rules(raw_input: &str) -> HashMap<&str, Vec<&str>> {
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let re = Regex::new(r"^(\w+\s\w+)\sbags|\d\s(\w+\s\w+)\sbags*").unwrap();

    for line in raw_input.lines() {
        let list: Vec<&str> = re.captures_iter(line).filter_map(|cap| {
            let group = cap.get(1).or(cap.get(2));
            match group {
                Some(bag) => Some(bag.as_str()),
                None => None,
            }
        }).collect();
        
        let bag = list[0];
        if !(bag == "shiny gold") {
            let rule: Vec<&str> = list[1..].to_vec();
            rules.insert(bag, rule);
        }
    }
    
    rules
}

fn count_bags<'a>(bag_rules: &HashMap<&'a str, Vec<&'a str>>, target: &'a str) -> usize {
    let mut count = 0;
    for (_, rules) in bag_rules.iter() {
        if contains_bag(bag_rules, rules, target) {
            count += 1;
        }
    }

    count
}

fn contains_bag<'a>(bag_rules: &HashMap<&'a str, Vec<&'a str>>, rule: &Vec<&'a str>, target: &'a str) -> bool {
    for &bag in rule.iter() {
        if bag == target || contains_bag(bag_rules, bag_rules.get(bag).unwrap(), target) {
            return true;
        }
    }

    false
}
