use std::collections::HashSet;
use std::fs::read_to_string;

type Person = HashSet<char>;
type Group = Vec<Person>;

fn read_groups(forms: &str) -> Vec<Group> {
    forms
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().collect::<Person>())
                .collect::<Group>()
        })
        .collect::<Vec<Group>>()
}

fn full_set() -> Person {
    ('a'..='z').collect::<Person>()
}

fn process_groups<F>(groups: &Vec<Group>, acc: Person, func: F) -> usize
where
    F: Fn(&Person, &Person) -> Person,
{
    groups
        .iter()
        .map(|group| group.iter().fold(acc.clone(), |a, person| func(&a, person)))
        .map(|group| group.len())
        .sum()
}

fn main() {
    let forms = read_to_string("./src/input.txt").expect("cannot read from file");
    let forms = forms.trim();
    let groups = read_groups(forms);

    println!(
        "{:?}",
        process_groups(&groups, Person::new(), |acc, person| {
            acc.union(&person).cloned().collect()
        })
    );
    println!(
        "{:?}",
        process_groups(&groups, full_set(), |acc, person| {
            acc.intersection(&person).cloned().collect()
        })
    );
}
