use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let values: Vec<_> = contents.lines().collect();

    let mut instructions: Vec<&str> = vec![];
    let mut args: Vec<u32> = vec![];

    for val in values {
        let command: Vec<_> = val.split(' ').collect();
        instructions.push(&command[0]);
        args.push(command[1].parse().expect("Can't parse number"));
    }

    let mut horizontal_position: u32 = 0;
    let mut vertical_position: u32 = 0;

    for (index, instruction) in instructions.iter().enumerate() {
        if instruction.to_string() == "forward" {
            horizontal_position += args[index];
        }
        if instruction.to_string() == "up" {
            vertical_position -= args[index];
        }
        if instruction.to_string() == "down" {
            vertical_position += args[index];
        }
    }

    println!("Part 1: {}", vertical_position * horizontal_position);

    horizontal_position = 0;
    vertical_position = 0;
    let mut aim: u32 = 0;

    for (index, instruction) in instructions.iter().enumerate() {
        if instruction.to_string() == "forward" {
            horizontal_position += args[index];
            vertical_position += aim * args[index];
        }
        if instruction.to_string() == "up" {
            aim -= args[index];
        }
        if instruction.to_string() == "down" {
            aim += args[index];
        }
    }

    println!("Part 2: {}", vertical_position * horizontal_position);
}
