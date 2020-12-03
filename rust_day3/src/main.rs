use std::fs::read_to_string;

fn main() {
    const SLOPES: [[usize; 2]; 5] = [
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2],
    ];

    let input = read_to_string("./src/input.txt").expect("cannot read file");
    let local_map: Vec<&str> = input.split('\n').collect();
    
    let part_one_count = count_trees(&local_map, 3, 1);
    println!("Part one answer is: {}", part_one_count);

    let mut part_two_product = 1i64;
    for slope in SLOPES.iter() {
        let (x_slope, y_slope) = (slope[0], slope[1]);
        let new = count_trees(&local_map, x_slope, y_slope) as i64;
        part_two_product = part_two_product * new;
    }
    println!("Part two answer is: {}", part_two_product);
}

fn count_trees(local_map: &Vec<&str>, x_slope: usize, y_slope: usize) -> i32 {
    let (mut tree_count, mut x_pos, mut y_pos) = (0i32, 0usize, 0usize);
    let map_width = local_map[0].chars().count();
    
    while y_pos < local_map.len() {
        let current_pos = local_map[y_pos as usize].chars().nth(x_pos as usize).unwrap();
        if current_pos == '#' {
            tree_count += 1
        }

        x_pos = (x_pos + x_slope) % map_width;
        y_pos = y_pos + y_slope;
    }

    tree_count
}
