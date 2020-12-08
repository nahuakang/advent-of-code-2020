use std::fs::read_to_string;

type Instruction<'a> = (&'a str, i32);
type Instructions<'a> = Vec<Instruction<'a>>;

fn main() {
    let raw = read_to_string("./src/input.txt").expect("cannot read file");
    let raw = raw.trim();
    let instructions = instructions_to_vec(raw);
    println!("{:?}", instructions);
    
    let part_1_acc = infinite_loop(&instructions);
    println!("{}", part_1_acc);

    // let part_2_acc = terminate_loop(&instructions);
    // println!("{}", part_2_acc);
}

fn instructions_to_vec(instructions: &str) -> Instructions {
    let mut ret = Vec::new();
    for instruction in instructions.lines() {
        let temp: Vec<&str> = instruction.split(" ").collect();
        let (command, info) = (temp[0], temp[1]);
        let (sign, digit_str) = info.split_at(1);
        
        let digit = match sign {
            "+" => digit_str.parse::<i32>().unwrap(),
            _ => -digit_str.parse::<i32>().unwrap()
        };
        
        ret.push(
            (command, digit),
        )
    }
    ret
}

fn infinite_loop(instructions: &Instructions) -> i32 {
    let (mut pos, mut acc) = (0i32, 0i32);
    let mut visited: Vec<i32> = Vec::new();

    loop {
        if visited.contains(&pos) {
            break;
        }
        visited.push(pos);
        println!("visited: {:?}", visited);
        println!();
        update_instruction(&instructions[pos as usize], &mut pos, &mut acc);
    }
    
    acc
}

fn update_instruction(instruction: &Instruction, curr_pos: &mut i32, acc: &mut i32) {
    // let mut new_acc = 0i32;
    // let new_pos: i32;
    
    match instruction.0 {
        "acc" => {
            *acc += instruction.1;
            *curr_pos += 1;
        }
        "jmp" => {
            *curr_pos += instruction.1;
        }
        _ => {
            *curr_pos += 1;
        }
    }
    println!("instruct {}, value {}", instruction.0, instruction.1);
    println!("new pos {}, new acc {}", *curr_pos, *acc);
}
