use std::fs::read_to_string;
use std::ops::Range;

fn main() {
    let input = read_to_string("./src/input.txt").expect("failure to read from file");
    let (rules, ticket, tickets) = parse_input(&input);
    let part_one_answer = part_one(&rules, &tickets);
    println!("Part one answer: {}", part_one_answer);
    let part_two_answer = part_two(&rules, &tickets, &ticket);
    println!("Part two answer: {}", part_two_answer);
}

type Rule = Vec<Range<usize>>;
type Ticket = Vec<usize>;
const DEPARTURE_COLS: Range<usize> = 0..6;

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut input = input.split("\n\n");
    let rules = input.next().unwrap().split("\n").map(parse_rule).collect::<Vec<Rule>>();
    let ticket = input.next().unwrap().split("\n").nth(1).map(parse_ticket).unwrap();
    let tickets = input.next().unwrap().split("\n").skip(1).map(parse_ticket).collect::<Vec<Ticket>>();

    (rules, ticket, tickets)
}

fn parse_rule(rule: &str) -> Rule {
    rule.split(':').nth(1).unwrap().split(" or ").map(|r| {
        let range = r.split('-').filter_map(|num| num.trim().parse().ok()).collect::<Vec<usize>>();
        range[0]..range[1]+1
    }).collect()
}

fn parse_ticket(ticket: &str) -> Ticket {
    ticket.split(',').filter_map(|n| n.parse().ok()).collect()
}

fn is_valid(num: usize, rules: &Vec<Rule>) -> bool {
    rules.iter().any(|rule| rule.iter().any(|range| range.contains(&num)))
}

fn part_one(rules: &Vec<Rule>, tickets: &Vec<Ticket>) -> usize {
    tickets.iter().map(|t| t.iter().map(|num| {
        if is_valid(*num, rules) {
            0
        } else {
            *num
        }
    })).flatten().sum()
}

fn part_two(rules: &Vec<Rule>, tickets: &Vec<Ticket>, ticket: &Ticket) -> usize {
    let valid_tickets = tickets.iter().filter(|t| t.iter().all(|&num| is_valid(num, rules))).collect::<Vec<&Ticket>>();

    let mut rules_per_col = (0..valid_tickets[0].len()).map(|col_i| {
        let col = valid_tickets.iter().filter_map(|t| t.get(col_i)).collect::<Vec<&usize>>();
        (0..rules.len()).filter(|&rule_i| {
            let rule = rules.get(rule_i).unwrap();
            col.iter().all(|num| rule.iter().any(|range| range.contains(num)))
        }).collect()
    }).collect::<Vec<Vec<usize>>>();

    let mut col_to_rule = vec![0; valid_tickets[0].len()];
    while rules_per_col.iter().any(|rules| rules.len() != 0) {
        let (i, rule) = rules_per_col.iter().enumerate().find(|(_, x)| x.len() == 1).map(|(i, rules)| (i, rules[0])).unwrap();
        col_to_rule[i] = rule;
        rules_per_col = rules_per_col.into_iter().map(|rules| rules.into_iter().filter(|&r| r != rule).collect()).collect();
    }
    col_to_rule.iter().enumerate().filter(|(_, col)| DEPARTURE_COLS.contains(*col)).map(|(pos, _)| ticket[pos]).product()
}
