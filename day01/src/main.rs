use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let values: Vec<u32> = contents
        .lines()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let values_length = values.len();

    let mut increment_count: u32 = 0;
    let mut previous_value: &u32 = &values[0];

    for val in &values {
        if val > previous_value {
            increment_count += 1;
        }
        previous_value = val;
    }

    println!("Part 1: {}", increment_count);

    let mut increment_count_2: u32 = 0;
    let mut high_index: usize = 2;
    let mut previous_sum: u32 = &values[0] + &values[1] + &values[2];

    while high_index < values_length {
        let sum_of_group = &values[high_index - 2] + &values[high_index - 1] + &values[high_index];

        if sum_of_group > previous_sum {
            increment_count_2 += 1;
        }

        previous_sum = sum_of_group;
        high_index += 1;
    }

    println!("Part 2: {}", increment_count_2);
}
