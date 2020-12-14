use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 answer: {}", part_one(input));
    println!("Part 2 answer: {}", part_two(input));
}

enum Instruction {
    Mask(u64, u64), // Xs, values
    Update(u64, u64), // mem loc, value
}

fn read_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            let (xs, values) = read_mask(&line[7..]);
            instructions.push(Instruction::Mask(xs, values));
        } else {
            let (memory_loc, value) = read_mem(line);
            instructions.push(Instruction::Update(memory_loc, value));
        }
    }
    instructions
}

fn read_mem(mem_instruction: &str) -> (u64, u64) {
    let mut mem_split = mem_instruction.split(" = ");
    let (left, right) = (mem_split.next().unwrap(), mem_split.next().unwrap());
    let end_idx = left.find("]").unwrap();
    let memory_loc = left[4..end_idx].parse().unwrap();
    let value = right.parse().unwrap();
    (memory_loc, value)
}

fn read_mask(mask_str: &str) -> (u64, u64) {
    let mut xs = 0;
    let mut values = 0;
    for c in mask_str.chars() {
        xs <<= 1;
        values <<= 1;
        match c {
            '1' => {
                values += 1;
            }
            '0' => {}
            'X' => {
                xs += 1;
            }
            _ => {
                unreachable!();
            }
        };
    }
    (xs, values)
}

fn part_one(input: &str) -> u64 {
    let instructions = read_input(input);
    let mut xs = 0;
    let mut values = 0;
    let mut memory = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(x, v) => {
                xs = x;
                values = v;
            }
            Instruction::Update(location, value) => {
                let e = memory.entry(location).or_default();
                *e = (value & xs) | values;
            }
        }
    }
    memory.values().sum()
}

fn part_two(input: &str) -> u64 {
    let instructions = read_input(input);
    let mut xs = 0;
    let mut values = 0;
    let mut memory = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(x, v) => {
                xs = x;
                values = v;
            }
            Instruction::Update(location, value) => {
                let unfloating = location | values;
                for x in get_all_floating(unfloating, xs) {
                    let e = memory.entry(x).or_default();
                    *e = value;
                }
            }
        }
    }
    memory.values().sum()
}

fn get_all_floating(mut location: u64, mut xs: u64) -> Vec<u64> {
    location = location & !xs;
    let mut floating_bits = Vec::new();
    let mut bit_index = 0;
    while xs > 0 {
        if xs & 1 == 1 {
            floating_bits.push(bit_index);
        }
        bit_index += 1;
        xs >>= 1;
    }

    let mut result = Vec::new();
    result.push(location);
    for floating in floating_bits {
        let bit = 1 << floating;

        for memory_location in result.clone().into_iter() {
            result.push(memory_location | bit);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    static INPUT1: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

    static INPUT2: &'static str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn part1() {
        assert_eq!(part_one(INPUT1), 165);
    }

    #[test]
    fn part2() {
        assert_eq!(part_two(INPUT2), 208);
    }
}