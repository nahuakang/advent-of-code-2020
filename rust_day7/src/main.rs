use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

type Rule<'a> = Vec<(&'a str, usize)>;
type Rules<'a> = HashMap<&'a str, Rule<'a>>;

fn main() {
    let luggage_rules = read_to_string("./src/input.txt").expect("cannot read from file");
    let luggage_rules: &str = luggage_rules.trim();
    let rules: Rules = get_bag_rules(luggage_rules);

    let part_1_count = count_bags(&rules, "shiny gold");
    println!("Part 1 count: {}", part_1_count);
    let part_2_count = count_required_bags(&rules, &rules["shiny gold"]);
    println!("Part 2 count: {}", part_2_count);
}

fn get_bag_rules(raw_input: &str) -> Rules {
    let mut rules: Rules = HashMap::new();
    let bag_parser = Regex::new(r"^(\w+\s\w+)\sbags*").unwrap();
    let rule_parse = Regex::new(r"(\d\s\w+\s\w+)\sbags*").unwrap();

    for line in raw_input.lines() {
        let bag = bag_parser
            .captures(line)
            .unwrap()
            .get(1)
            .map_or("", |m| m.as_str());

        let rule: Vec<(&str, usize)> = rule_parse
            .captures_iter(line)
            .filter_map(|cap| {
                let group = cap.get(1).or(cap.get(2));
                match group {
                    Some(bag) => Some(bag.as_str()),
                    None => None,
                }
            }) // Note that filter_map already unwrapped the Option
            .map(|input| {
                let mut iter = input.splitn(2, ' ');
                let num = iter.next().unwrap().parse::<usize>().unwrap();
                let bag = iter.next().unwrap();
                (bag, num)
            })
            .collect();

        rules.insert(bag, rule);
    }
    rules
}

fn count_bags(bag_rules: &Rules, target: &str) -> usize {
    bag_rules
        .iter()
        .map(|(_, rule)| contains_bag(bag_rules, rule, target) as usize)
        .sum()
}

fn contains_bag(bag_rules: &Rules, rule: &Rule, target: &str) -> bool {
    for &bag in rule.iter() {
        if bag.0 == target || contains_bag(bag_rules, bag_rules.get(bag.0).unwrap(), target) {
            return true;
        }
    }

    false
}

fn count_required_bags(bag_rules: &Rules, rule: &Rule) -> i32 {
    if rule.is_empty() {
        0i32
    } else {
        let mut count = 0i32;

        for &bag in rule.iter() {
            count = count
                + bag.1 as i32
                + bag.1 as i32 * count_required_bags(bag_rules, &bag_rules[bag.0]);
        }

        count
    }
}
