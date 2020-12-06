use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let forms = read_to_string("./src/input.txt").expect("cannot read from file");
    let forms = forms.trim(); 
    let groups = forms.split("\n\n").collect::<Vec<&str>>();
    println!("{:?}", count_all_questions(&groups, count_group_questions_union));
    println!("{:?}", count_all_questions(&groups, count_group_questions_intersect));
}

fn count_all_questions(groups: &Vec<&str>, f: fn(&str) -> usize) -> usize {
    let mut total: usize = 0;
    for &group in groups.iter() {
        println!();
        println!("Group is: \n{}", group);
        println!("Count is: {}", f(group));
        total += f(group);
    }
    total
}

fn count_group_questions_union(group: &str) -> usize {
    let individuals = group.split("\n").collect::<Vec<&str>>();
    let mut total = HashSet::new();
    for individual in individuals.iter() {
        for c in individual.chars() {
            total.insert(c);
        }
    }
    total.len()
}

fn count_group_questions_intersect(group: &str) -> usize {
    let individuals = group.split("\n").collect::<Vec<&str>>();
    
    let mut sets = Vec::new();
    for &individual in individuals.iter() {
        println!("Individual is: {:?}", individual.chars().collect::<HashSet<_>>());
        sets.push(individual.chars().collect::<HashSet<_>>());
    }

    if sets.len() == 1 {
        return sets[0].len();
    }
    
    let mut first = sets[0].clone();
    for set in sets.iter().skip(1) {
        first = first.intersection(&set).cloned().collect();
    }

    first.len()
}
