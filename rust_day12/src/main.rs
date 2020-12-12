use std::fs::read_to_string;

fn main() {
    let instructions = read_to_string("./src/input.txt").expect("cannot read from file");
    let instructions = load_instructions(&instructions);
    println!("Manhattan distance: {}", part_one(&instructions));
    println!("Manhattan distance: {}", part_two(&instructions));
}

type Instruction<'a> = (&'a str, isize);

fn load_instructions<'a>(raw_input: &'a String) -> Vec<Instruction<'a>> {
    raw_input.trim().lines().map(|line| {
        let (direction, value) = line.split_at(1);
        (direction, value.parse::<isize>().unwrap())
    }).collect()
}

// Direction to num => N: 0, E: 1, S: 2, W: 3
fn part_one(instructions: &Vec<Instruction>) -> usize {
    const NUM_DIRS: usize = 4;
    let mut direction = 1; 
    let mut y = 0;
    let mut x = 0;

    for (instruction, value) in instructions.clone() {
        match instruction {
            "N" => y += value,
            "E" => x += value,
            "S" => y -= value,
            "W" => x -= value,
            "L" => direction = (direction + NUM_DIRS - value as usize / 90) % NUM_DIRS,
            "R" => direction = (direction + NUM_DIRS + value as usize / 90) % NUM_DIRS,
            "F" => match direction {
                0 => y += value,
                1 => x += value,
                2 => y -= value,
                3 => x -= value,
                _ => panic!("Invalid direction"),
            },
            _ => panic!("Invalid instruction"),
        }
    }

    return (y.abs() + x.abs()) as usize;
}

fn part_two(instructions: &Vec<Instruction>) -> usize {
    let mut pos = (0, 0);
    let mut way_point = (10, 1);

    for (instruction, value) in instructions.clone() {
        match instruction {
            "N" => way_point.1 += value,
            "E" => way_point.0 += value,
            "S" => way_point.1 -= value,
            "W" => way_point.0 -= value,
            "L" => {
                for _i in 0..(value / 90) {
                    way_point = (-way_point.1, way_point.0);
                }
            },
            "R" => {
                for _i in 0..(value / 90) {
                    way_point = (way_point.1, -way_point.0);
                }
            },
            "F" => pos = (pos.0 + way_point.0 * value, pos.1 + way_point.1 * value),
            _ => panic!("Invalid instruction"),
        }
    }

    return (pos.0.abs() + pos.1.abs()) as usize;
}


#[cfg(test)]
mod tests {
    use super::{read_to_string, load_instructions, part_one, part_two};

    #[test]
    fn test_part_one() {
        let instructions = read_to_string("./src/test_input.txt").expect("cannot read from file");
        let instructions = load_instructions(&instructions);
        assert_eq!(part_one(&instructions), 25);
    }

    #[test]
    fn test_part_two() {
        let instructions = read_to_string("./src/test_input.txt").expect("cannot read from file");
        let instructions = load_instructions(&instructions);
        assert_eq!(part_two(&instructions), 286);
    }
}