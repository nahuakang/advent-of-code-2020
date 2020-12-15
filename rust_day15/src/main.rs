fn main() {
    let input = vec![9, 6, 0, 10, 18, 2, 1];

    let last_num = find_nth_number(2_020, &input);
    println!("2020th number spoken is: {}", last_num);

    let last_num = find_nth_number(30_000_000, &input);
    println!("30000000th number spoken is: {}", last_num);
}

fn find_nth_number(n: usize, input: &[usize]) -> usize {
    let mut last_spoken_account = vec![0; n];
    input
        .iter()
        .enumerate()
        .for_each(|(idx, num)| last_spoken_account[*num] = idx + 1);
    let mut last_number_spoken = input[input.len() - 1];

    for step in input.len()..n {
        let last_spoken_on = last_spoken_account[last_number_spoken];
        last_spoken_account[last_number_spoken] = step;
        last_number_spoken = if last_spoken_on != 0 {
            step - last_spoken_on
        } else {
            0
        }
    }

    last_number_spoken
}

#[cfg(test)]
mod tests {
    use super::find_nth_number;
    #[test]
    fn test_part_one() {
        let input0 = vec![0, 3, 6];
        assert_eq!(find_nth_number(2020, &input0), 436);
        let input1 = vec![1, 3, 2];
        assert_eq!(find_nth_number(2020, &input1), 1);
        let input2 = vec![2, 1, 3];
        assert_eq!(find_nth_number(2020, &input2), 10);
        let input3 = vec![1, 2, 3];
        assert_eq!(find_nth_number(2020, &input3), 27);
        let input4 = vec![2, 3, 1];
        assert_eq!(find_nth_number(2020, &input4), 78);
        let input5 = vec![3, 2, 1];
        assert_eq!(find_nth_number(2020, &input5), 438);
        let input6 = vec![3, 1, 2];
        assert_eq!(find_nth_number(2020, &input6), 1_836);
    }

    #[test]
    fn test_part_two() {
        let input0 = vec![0, 3, 6];
        assert_eq!(find_nth_number(30_000_000, &input0), 175_594);
        let input1 = vec![1, 3, 2];
        assert_eq!(find_nth_number(30_000_000, &input1), 2_578);
        let input2 = vec![2, 1, 3];
        assert_eq!(find_nth_number(30_000_000, &input2), 3_544_142);
        let input3 = vec![1, 2, 3];
        assert_eq!(find_nth_number(30_000_000, &input3), 261_214);
        let input4 = vec![2, 3, 1];
        assert_eq!(find_nth_number(30_000_000, &input4), 6_895_259);
        let input5 = vec![3, 2, 1];
        assert_eq!(find_nth_number(30_000_000, &input5), 18);
        let input6 = vec![3, 1, 2];
        assert_eq!(find_nth_number(30_000_000, &input6), 362);
    }
}
