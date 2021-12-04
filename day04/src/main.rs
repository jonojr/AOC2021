use std::fs;

struct Board {
    board: [[(u32, bool); 5]; 5],
    won: bool,
}

impl Board {
    fn has_won(&mut self) -> bool {
        if self.won {
            return false;
        }
        let mut won = false;
        won |= self.check_row(0);
        won |= self.check_row(1);
        won |= self.check_row(2);
        won |= self.check_row(3);
        won |= self.check_row(4);

        won |= self.check_column(0);
        won |= self.check_column(1);
        won |= self.check_column(2);
        won |= self.check_column(3);
        won |= self.check_column(4);

        self.won = won;
        return won;
    }
    fn check_row(&self, row_id: usize) -> bool {
        let mut all_true = true;
        for cell in self.board[row_id] {
            all_true &= cell.1;
        }
        return all_true;
    }
    fn check_column(&self, col_id: usize) -> bool {
        let mut all_true = true;
        for row_id in 0..5 {
            all_true &= self.board[row_id][col_id].1;
        }
        return all_true;
    }
    fn mark_cells(&mut self, value: u32) {
        for row_id in 0..5 {
            for col_id in 0..5 {
                if self.board[row_id][col_id].0 == value {
                    self.board[row_id][col_id] = (value, true);
                }
            }
        }
    }
    fn get_score(&self, most_recent_value: u32) -> u32 {
        let mut sum_false:u32 = 0;

        for row in self.board {
            for cell in row {
                // println!("{:?}", cell);
                if !cell.1 {
                    sum_false += cell.0;
                    // println!("{:?}", cell)
                }
            }
        }

        return sum_false * most_recent_value;
    }
}

fn string_vec_to_board_array(string_input:Vec<&str>) -> [[(u32, bool);5];5]{
    let mut arrays:Vec<[(u32, bool);5]> = Vec::with_capacity(5);

    for line in string_input {
        let split_line: Vec<&str> = line.split_whitespace().collect();

        let u32_line: Vec<(u32, bool)> = split_line.into_iter()
            .map(|x| (usize::from_str_radix(x, 10).expect("Error converting val in string to board array") as u32, false))
            .collect();

        let array_line:[(u32, bool);5] = [u32_line[0], u32_line[1], u32_line[2], u32_line[3], u32_line[4]];
        arrays.push(array_line);
    }

    return [arrays[0], arrays[1], arrays[2], arrays[3], arrays[4]];
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let file_blocks: Vec<_> = contents.split("\n\n").collect();

    let input_values: Vec<_> = file_blocks[0].split(",").collect();

    let random_numbers: Vec<u32> = input_values
        .into_iter()
        .map(|x| usize::from_str_radix(x, 10).expect("Error converting val") as u32)
        .collect();


    let mut boards: Vec<_> = vec![];

    for i in 1..file_blocks.len() {
        let board_lines:Vec<_> = file_blocks[i].lines().collect();
        let board = string_vec_to_board_array(board_lines);

        let created_board = Board {board, won:false};
        boards.push(created_board);
    }

    'outer: for number in random_numbers{
        println!("Number: {}", number);
        for board_index in 0..boards.len() {
            boards[board_index].mark_cells(number);

            if boards[board_index].has_won() {
                println!("A BOARD HAS WON!!!");
                println!("{:?}", boards[board_index].get_score(number));
                // break 'outer;
            }
        }
    }

}
