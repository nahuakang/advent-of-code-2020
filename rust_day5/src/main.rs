use std::fs::read_to_string;

fn main() {
    let seats = read_to_string("./src/input.txt").expect("cannot read file to string");
    let ids = get_seat_ids(seats.lines().collect::<Vec<&str>>());
    println!("{}", ids.iter().max().unwrap()); // Should be 816

    let missing_id = find_missing_id(&ids);
    println!("{}", missing_id); // Should be 539
}

fn find_missing_id(seat_ids: &Vec<i32>) -> i32 {
    let mut seat_ids = seat_ids.clone();
    seat_ids.sort();
    let missing_id = seat_ids.windows(2).find(|window| !(window[0] + 1 == window[1])).unwrap();
    let missing_id = missing_id[1] - 1;
    missing_id
}

fn get_seat_ids(seats: Vec<&str>) -> Vec<i32> {
    seats.iter().map(|hint| get_seat_id(hint)).collect::<Vec<i32>>()
}

fn get_seat_id(seat_hint: &str) -> i32 {
    let (row_hint, col_hint) = seat_hint.split_at(7);
    let row = get_row(
        row_hint
            .chars()
            .collect::<Vec<char>>(),
    );
    let col = get_col(
        col_hint
            .chars()
            .collect::<Vec<char>>(),
    );
    row * 8 + col
}

fn get_row(row_hint: Vec<char>) -> i32 {
    let (mut lower, mut upper) = (0i32, 127);
    for &hint in row_hint.iter().take(6) {
        update_limit(&mut lower, &mut upper, hint);
    }
    match *row_hint.iter().last().unwrap() {
        'F' => lower,
        _ => upper,
    }
}

fn get_col(col_hint: Vec<char>) -> i32 {
    let (mut lower, mut upper) = (0i32, 7);
    for &hint in col_hint.iter().take(2) {
        update_limit(&mut lower, &mut upper, hint);
    }
    match *col_hint.iter().last().unwrap() {
        'L' => lower,
        _ => upper,
    }
}

fn update_limit(lower: &mut i32, upper: &mut i32, pos: char) {
    match pos {
        'F' | 'L' => *upper = upper.clone() - (1 + upper.clone() - lower.clone()) / 2,
        'B' | _ => *lower = lower.clone() + (1 + upper.clone() - lower.clone()) /2,
    }
}
