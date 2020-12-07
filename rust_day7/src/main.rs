use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let luggage_rules = read_to_string("./src/input.txt").expect("cannot read from file");
    let luggage_rules: &str = luggage_rules.trim();

    let mut rules: HashMap<&str, Vec<&str>> = get_bag_rules(luggage_rules);

    let count = count_bags(&mut rules, vec!("shiny gold"), 0);
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

fn count_bags<'a>(bag_rules: &mut HashMap<&'a str, Vec<&'a str>>, targets: Vec<&'a str>, mut count: usize) -> usize {
    if targets.len() == 0 {
        return count;
    }

    let mut new_targets = Vec::<&str>::new();

    for (bag, rules) in bag_rules.iter_mut() {
        for target in targets.iter() {
            if rules.contains(target) && !new_targets.contains(target) {
                new_targets.push(bag);
                count += 1;
            }
        }
    }

    for bag in targets.iter() {
        bag_rules.remove(bag);
    }

    count_bags(bag_rules, new_targets, count)
}
