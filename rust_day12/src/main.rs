use std::fs::read_to_string;

fn main() {
    let instructions = read_to_string("./src/input.txt").expect("cannot read from file");
    let instructions = instructions.trim();
    let mut ship = Ship::new(instructions);
    ship.sail();
    println!("Manhattan distance: {}", ship.get_manhattan_distance());
}

struct Ship<'a> {
    instructions: &'a str,
    direction: Direction,
    east: isize,
    west: isize,
    north: isize,
    south: isize,
}

impl<'a> Ship<'a> {
    fn new(instructions: &'a str) -> Self {
        Self {
            instructions,
            direction: Direction::East,
            east: 0,
            west: 0,
            north: 0,
            south: 0,
        }
    }

    fn sail(&mut self) {
        for instruction in self.instructions.lines() {
            let (dir, val) = instruction.split_at(1);
            let val = val.parse::<isize>().unwrap();
            match dir {
                "F" => {
                    let direction = self.direction;
                    self.move_once(direction, val);
                },
                "L" | "R" => self.change_direction(dir, val),
                "E" | "W" | "N" | "S" => {
                    self.move_once(self.str_to_direction(dir), val)
                },
                _ => panic!("Invalid direction input"),
            } 
        }
    }

    fn get_manhattan_distance(&self) -> isize {
        (self.east - self.west).abs() + (self.north - self.south).abs()
    }

    fn move_once(&mut self, direction: Direction, val: isize) {
        match direction {
            Direction::East => self.east += val,
            Direction::West => self.west += val,
            Direction::North => self.north += val,
            Direction::South => self.south += val,
        }
    }

    fn change_direction(&mut self, direction: &str, value: isize) {        
        let mut current_direction = self.direction_to_num();
        match direction {
            "L" => current_direction = (current_direction + 4 - value / 90) % 4,
            "R" => current_direction = (current_direction + 4 + value / 90) % 4,
            _ => panic!("Invalid change direction instruction"),
        }
        self.direction = self.num_to_direction(current_direction);
    }

    fn str_to_direction(&self, direction: &str) -> Direction {
        match direction {
            "E" => Direction::East,
            "W" => Direction::West,
            "N" => Direction::North,
            "S" => Direction::South,
            _ => panic!("Invalid direction str"),
        }
    }

    fn direction_to_num(&self) -> isize {
        match self.direction {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    fn num_to_direction(&self, num: isize) -> Direction {
        match num {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Invalid direction num"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    North,
    South,
    West,
}

#[cfg(test)]
mod tests {
    use super::{read_to_string, Ship};

    #[test]
    fn test_part_one() {
        let instructions = read_to_string("./src/test_input.txt").expect("cannot read from file");
        let instructions = instructions.trim();
        let mut ship = Ship::new(instructions);
        ship.sail();
        assert_eq!(ship.get_manhattan_distance(), 25);
    }
}