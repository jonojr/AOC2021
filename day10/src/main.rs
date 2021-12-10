use std::fs;

fn get_closing_char(opening_char: char) -> char {
    return if opening_char == '(' {
        ')'
    } else if opening_char == '[' {
        ']'
    } else if opening_char == '{' {
        '}'
    } else if opening_char == '<' {
        '>'
    } else {
        'F'
    }
}

fn is_opening_char(given_char: char) -> bool {
    return if given_char == '(' {
        true
    } else if given_char == '[' {
        true
    } else if given_char == '{' {
        true
    } else if given_char == '<' {
        true
    } else {
        false
    }
}

fn get_error_char_value (error_char: char) -> u32 {
    return if error_char == ')' {
        3
    } else if error_char == ']' {
        57
    } else if error_char == '}' {
        1197
    } else if error_char == '>' {
        25137
    } else {
        0
    }
}

fn get_completion_char_value(completion_char: char) -> u64 {
    return if completion_char == ')' {
        1
    } else if completion_char == ']' {
        2
    } else if completion_char == '}' {
        3
    } else if completion_char == '>' {
        4
    } else {
        0
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let chunk_lines: Vec<Vec<char>> = file_contents.lines().map(|x| x.chars().collect::<Vec<char>>()).collect();

    let mut score:u32 = 0;

    let mut line_completions:Vec<Vec<char>> =  vec!();

    for chunk_line in chunk_lines {
        let mut closing_chars: Vec<char> = vec!();
        let mut failed = false;

        for current_char in chunk_line {
            if is_opening_char(current_char){
                closing_chars.push(get_closing_char(current_char));
            }
            else {
                let expected_char = closing_chars.pop().unwrap();

                if current_char != expected_char {
                    score += get_error_char_value(current_char);
                    failed = true;
                    break;
                }
            }
        }
        if !failed {
            line_completions.push(closing_chars);
        }
    }

    println!("Part 1: {:?}", score);

    let mut completion_scores:Vec<u64> = vec!();
    for mut line in line_completions {
        let mut line_value:u64 = 0;
        while line.len() > 0 {
            let completion_char = line.pop().unwrap();
            line_value *= 5;
            line_value += get_completion_char_value(completion_char);
        }
        completion_scores.push(line_value);
    }

    completion_scores.sort();

    println!("Part 2: {:?}", completion_scores[completion_scores.len() / 2]);
}
