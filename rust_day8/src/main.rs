use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let raw = read_to_string("./src/input.txt").expect("cannot read file");
    let raw = raw.trim();
    let part_1_count = part_one(raw);
    println!("Part 1 answer: {}", part_1_count);
    let part_2_count = part_two(raw);
    println!("part 2 answer: {}", part_2_count);
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instruction {
    fn is_nop(&self) -> bool {
        match self {
            Instruction::Nop(_) => true,
            _ => false,
        }
    }

    fn is_jmp(&self) -> bool {
        match self {
            Instruction::Jmp(_) => true,
            _ => false,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(raw_instruction: &str) -> Result<Self, Self::Err> {
        let mut parts = raw_instruction.split_whitespace().map(str::trim);
        
        match(parts.next(), parts.next()) {
            (Some(instruction), Some(value)) => {
                let parsed_value = value.parse::<isize>().unwrap();

                match instruction {
                    "nop" => Ok(Instruction::Nop(parsed_value)),
                    "acc" => Ok(Instruction::Acc(parsed_value)),
                    "jmp" => Ok(Instruction::Jmp(parsed_value)),
                    _ => Err(format!("invalid instruction {}", raw_instruction)),
                }
            },
            _ => Err(format!("invalid instruction {}", raw_instruction)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HaltStatus {
    Infinite,
    Finite,
}

struct GameConsoleDebugger {
    acc: isize,
    instructions: Vec<Instruction>,
    pos: usize,
}

impl GameConsoleDebugger {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            acc: 0,
            instructions,
            pos: 0,
        }
    }

    fn is_infinite_or_finite(&mut self) -> HaltStatus {
        let mut visited_executions = HashSet::<usize>::new();

        while self.pos < self.instructions.len() {
            let instruction = self.instructions[self.pos];
            
            let (new_pos, new_acc) = match instruction {
                Instruction::Jmp(offset) => (self.pos as isize + offset, self.acc),
                Instruction::Acc(change) => (self.pos as isize + 1, self.acc + change),
                Instruction::Nop(_) => (self.pos as isize + 1, self.acc),
            };

            if !visited_executions.insert(self.pos) {
                return HaltStatus::Infinite;
            }

            self.acc = new_acc;
            self.pos = new_pos as usize;
        }

        HaltStatus::Finite
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        Instruction::from_str(line.trim()).unwrap()
    }).collect::<Vec<_>>()
}

fn part_one(input: &str) -> isize {
    let instructions = parse_instructions(input);
    let mut gc_debugger = GameConsoleDebugger::new(instructions);
    gc_debugger.is_infinite_or_finite();
    gc_debugger.acc
}

fn part_two(input: &str) -> isize {
    let instructions = parse_instructions(input);
    instructions.iter().enumerate().find_map(|(idx, &instruction)| {
        if instruction.is_jmp() || instruction.is_nop() {
            let mut new_instructions = instructions.clone();
            
            match instruction {
                Instruction::Nop(v) => {
                    new_instructions[idx] = Instruction::Jmp(v);
                }
                Instruction::Jmp(v) => {
                    new_instructions[idx] = Instruction::Nop(v);
                }
                _ => {}
            }

            let mut gc_debugger = GameConsoleDebugger::new(new_instructions);
            let status = gc_debugger.is_infinite_or_finite();
            if status == HaltStatus::Finite {
                Some(gc_debugger.acc)
            } else {
                None
            }
        } else {
            None
        }
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    const INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 5);
    }

    #[test]
    fn test_star_two() {assert_eq!(part_two(INPUT), 8);}
}
