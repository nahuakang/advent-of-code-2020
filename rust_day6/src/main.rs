use std::collections::HashSet;
use std::fs::read_to_string;

struct Group<'a> {
    all_ascii_chars: HashSet<char>,
    individuals: &'a str,
}

impl<'a> Group<'a> {
    fn new(group: &'a str) -> Self {
        let chars = ('a'..='z').filter(|c| c.is_alphabetic()).collect::<HashSet<char>>();
        Self{
            all_ascii_chars: chars,
            individuals: group,
        }
    }

    fn union_count(&self) -> usize {
        let mut union = HashSet::new();
        for c in self.individuals.chars() {
            if !c.is_ascii_alphabetic() {
                continue;
            }
            union.insert(c);
        }
        union.len()
    }

    fn intersection_count(&self) -> usize {
        let mut intersect = self.all_ascii_chars.iter().cloned().collect::<HashSet<char>>();
        for individual in self.individuals.split("\n") {
            let mut cur = HashSet::new();
            for c in individual.chars() {
                cur.insert(c);
            }

            intersect = intersect.intersection(&cur).cloned().collect();
        }

        intersect.len()
    }
}

fn main() {
    let forms = read_to_string("./src/input.txt").expect("cannot read from file");
    let forms = forms.trim(); 
    
    println!("{:?}", forms.split("\n\n").map(|group| Group::new(group)).map(|group| group.union_count()).sum::<usize>());
    println!("{:?}", forms.split("\n\n").map(|group| Group::new(group)).map(|group| group.intersection_count()).sum::<usize>());
}
