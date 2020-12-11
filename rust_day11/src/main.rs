use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./src/input.txt").expect("cannot read from file");
    let raw_input =  raw_input.trim();
    let mut waiting_area = Seats::new(raw_input);
    waiting_area.allocate_seats_until_stabilization();
    println!("Part one answer: {}", waiting_area.count_occupied_seats());
    let mut waiting_area = Seats::new(raw_input);
    waiting_area.allocate_seats_until_stabilization_2();
    println!("Part two answer: {}", waiting_area.count_occupied_seats());
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Status {
    Floor,
    Empty,
    Occupied,
}

impl Status {
    fn is_occupied(&self) -> bool {
        match self {
            Status::Occupied => true,
            _ => false,
        }
    }
}

type Seat = (usize, usize);

#[derive(Debug)]
struct Seats {
    seats: HashMap<Seat, Status>,
    rows: usize,
    cols: usize,
}

impl Seats {
    fn new(raw_seats_input: &str) -> Self {
        let mut seats = HashMap::<Seat, Status>::new();
        let rows: usize = raw_seats_input.lines().count();
        let cols: usize =raw_seats_input.lines().next().unwrap().chars().count();
        
        for (i, row) in raw_seats_input.lines().enumerate() {
            for (j, seat) in row.chars().enumerate() {
                let status = match seat {
                    '.' => Status::Floor,
                    'L' => Status::Empty,
                    _ => panic!("Invalid seat information."),
                };
                seats.insert((i, j), status);
            }
        }

        Self {seats, rows, cols}
    }

    fn count_occupied_seats(&self) -> usize {
        self.seats.values().map(|&v| v.is_occupied() as usize).sum()
    }

    fn equals(&self, new_seats: &HashMap<Seat, Status>) -> bool {
        self.seats.len() == new_seats.len() && self.seats.iter().all(|(k, v)| {
            new_seats.get(k).unwrap() == v
        })
    }

    fn allocate_seats_until_stabilization(&mut self) {
        let mut new_seats = self.get_new_seats();
        while !self.equals(&new_seats) {
            self.seats = new_seats;
            new_seats = self.get_new_seats();
        }
    }

    fn get_new_seats(&self) -> HashMap<Seat, Status>{
        let mut new_seats = HashMap::<Seat, Status>::new();
        for (&seat, &status) in self.seats.iter() {
            let new_status = self.apply_seat_rules_1(seat, status);
            new_seats.insert(seat, new_status);
        }

        new_seats
    }

    fn apply_seat_rules_1(&self, seat: Seat, current_status: Status) -> Status {
        let adjacent_occupied_count: usize = Seats::adjacent_seats(seat.0 as isize, seat.1 as isize).iter().map(|neighbor| {
            match self.seats.get(&(neighbor.0 as usize, neighbor.1 as usize)) {
                Some(&status) => match status {
                    Status::Empty | Status::Floor => 0,
                    Status::Occupied => 1,
                }
                None => 0,
            }
        }).sum();

        match adjacent_occupied_count {
            0 if current_status != Status::Occupied && current_status != Status::Floor => Status::Occupied,
            4..=8 if current_status == Status::Occupied => Status::Empty,
            _ => current_status,
        }
    }

    fn adjacent_seats(x: isize, y: isize) -> [(isize, isize); 8] {
        [
            (x-1, y-1), (x, y-1), (x+1, y-1),
            (x-1, y),             (x+1, y),
            (x-1, y+1), (x, y+1), (x+1, y+1),
        ]
    }

    fn allocate_seats_until_stabilization_2(&mut self) {
        let mut new_seats = self.get_new_seats_2();
        while !self.equals(&new_seats) {
            self.seats = new_seats;
            new_seats = self.get_new_seats_2();
        }
    }

    fn get_new_seats_2(&self) -> HashMap<Seat, Status>{
        let mut new_seats = HashMap::<Seat, Status>::new();
        for (&seat, &status) in self.seats.iter() {
            let new_status = self.apply_seat_rules_2(seat, status);
            new_seats.insert(seat, new_status);
        }

        new_seats
    }

    fn apply_seat_rules_2(&self, seat: Seat, current_status: Status) -> Status {
        match self.count_occupied_direction(seat) {
            0 if current_status != Status::Occupied && current_status != Status::Floor => Status::Occupied,
            5..=8 if current_status == Status::Occupied => Status::Empty,
            _ => current_status,
        }
    }

    fn count_occupied_direction(&self, seat: Seat) -> usize {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .fold(0, |mut acc, dir| {
            if self.find_occupied_seats_in_direction(seat, *dir) {
                acc += 1
            }

            acc
        })
    }

    fn find_occupied_seats_in_direction(&self, seat: Seat, direction: (isize, isize)) -> bool {
        let r_len = self.rows as isize;
        let c_len = self.cols as isize;

        let (r, c) = ((seat.0 as isize + direction.0) , (seat.1 as isize + direction.1));

        if r < 0 || r >= r_len || c < 0 || c >= c_len {
            return false;
        }

        let current_seat = self.seats.get(&(r as usize, c as usize));

        match current_seat {
            Some(&Status::Empty) => return false,
            Some(&Status::Occupied) => return true,
            _ => (),
        }

        self.find_occupied_seats_in_direction((r as usize, c as usize), direction)
    }
}

#[cfg(test)]
mod tests {
    use super::{Seats};
    const INPUT: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
    #[test]
    fn test_part_one() {
        let mut waiting_area = Seats::new(INPUT);
        waiting_area.allocate_seats_until_stabilization();
        println!("current seats: {:?}", waiting_area.seats);
        assert_eq!(waiting_area.count_occupied_seats(), 37);
    }

    #[test]
    fn test_part_two() {
        let mut waiting_area = Seats::new(INPUT);
        waiting_area.allocate_seats_until_stabilization_2();
        println!("current seats: {:?}", waiting_area.seats);
        assert_eq!(waiting_area.count_occupied_seats(), 26);
    }
}
