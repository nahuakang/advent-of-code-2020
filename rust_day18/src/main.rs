use std::collections::VecDeque;
use std::str::Chars;

fn main() {
    let input = load_input();
    let part1_answer = part1(&input);
    println!("Part 1 answer: {}", part1_answer);
    let part2_answer = part2(&input);
    println!("Part 2 answer: {}", part2_answer);
}

fn part1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| evaluate(&mut line.chars(), part1_evaluator))
        .sum()
}

fn part2(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| evaluate(&mut line.chars(), part2_evaluator))
        .sum()
}

#[derive(PartialEq, Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

fn load_input() -> Vec<&'static str> {
    let input = include_str!("input.txt").trim();
    input.lines().collect::<Vec<&str>>()
}

type Evaluator = fn(VecDeque<usize>, VecDeque<Op>) -> usize;

fn evaluate(line: &mut Chars, evaluator: Evaluator) -> usize {
    let mut vals = VecDeque::<usize>::new();
    let mut ops = VecDeque::<Op>::new();

    while let Some(char) = line.next() {
        match char {
            ' ' => continue,
            '*' => ops.push_back(Op::Mul),
            '+' => ops.push_back(Op::Add),
            '(' => vals.push_back(evaluate(line, evaluator)),
            ')' => break,
            c if '0' <= c && '9' >= c => vals.push_back(c as usize - '0' as usize),
            _ => unreachable!(),
        }
    }
    evaluator(vals, ops)
}

fn part1_evaluator(mut vals: VecDeque<usize>, mut ops: VecDeque<Op>) -> usize {
    while let Some(op) = ops.pop_front() {
        let a = vals.pop_front().expect("pls");
        let b = vals.pop_front().expect("pls");
        vals.push_front(op.apply(a, b));
    }
    vals.pop_front().expect("shoud be one")
}

fn part2_evaluator(mut vals: VecDeque<usize>, mut ops: VecDeque<Op>) -> usize {
    while let Some(idx) = ops
        .iter()
        .enumerate()
        .find(|(_, op)| **op == Op::Add)
        .map(|(i, _)| i)
    {
        let a = vals.remove(idx).unwrap();
        let b = vals.remove(idx).unwrap();
        vals.insert(idx, a + b);
        ops.remove(idx);
    }
    vals.iter().product()
}

#[cfg(test)]
mod tests {
    use super::{evaluate, part1_evaluator, part2_evaluator};

    #[test]
    fn test_example_simple() {
        let s = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(evaluate(&mut s.chars(), part1_evaluator), 71);

        let s = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(evaluate(&mut s.chars(), part1_evaluator), 51);
    }

    #[test]
    fn test_example_adv() {
        let s = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(evaluate(&mut s.chars(), part2_evaluator), 51);

        let s = "2 * 3 + (4 * 5)";
        assert_eq!(evaluate(&mut s.chars(), part2_evaluator), 46);

        let s = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(evaluate(&mut s.chars(), part2_evaluator), 231);
    }
}
