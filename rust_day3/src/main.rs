use std::fs::read_to_string;

fn main() {
    const SLOPES: [[usize; 2]; 5] = [
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2],
    ];

    let local_map = read_to_string("./src/input.txt").expect("cannot read file");
    // let local_map: Vec<&str> = input.split('\n').collect();
    
    let part_one_count = count_trees(&local_map, 3, 1);
    println!("Part one answer is: {}", part_one_count);

    let part_two_product: i64 = SLOPES
        .iter()
        .map(|slopes| count_trees(&local_map, slopes[0], slopes[1]) as i64)
        .product();
    println!("Part two answer is: {}", part_two_product);
}

fn count_trees(local_map: &str, x_slope: usize, y_slope: usize) -> i32 {
    local_map.lines()
        .step_by(y_slope)
        .enumerate()
        .filter(|(step, line)| line.chars().nth(step * x_slope % line.len()).unwrap() == '#')
        .count() as i32
}
