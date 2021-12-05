use std::fs;

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        return self.y1 == self.y2;
    }
    fn is_vertical(&self) -> bool {
        return self.x1 == self.x2;
    }
    fn is_diagonal(&self) -> bool {
        let x_dif: i32 = self.x1 as i32 - self.x2 as i32;
        let y_dif: i32 = self.y1 as i32 - self.y2 as i32;

        return x_dif.abs() == y_dif.abs();
    }
    fn slope(&self) -> i32 {
        let x_dif = self.x1 as i32 - self.x2 as i32;
        return x_dif.abs();
    }
    fn from_str(&mut self, defintion: &str) {
        let points: Vec<_> = defintion.split(" -> ").collect();
        let point1: Vec<_> = points[0].split(",").collect();

        self.x1 = usize::from_str_radix(point1[0], 10)
            .expect("Error converting val in string to board array");
        self.y1 = usize::from_str_radix(point1[1], 10)
            .expect("Error converting val in string to board array");

        let point2: Vec<_> = points[1].split(",").collect();
        self.x2 = usize::from_str_radix(point2[0], 10)
            .expect("Error converting val in string to board array");
        self.y2 = usize::from_str_radix(point2[1], 10)
            .expect("Error converting val in string to board array");
    }
}

struct Board {
    board: Vec<Vec<u32>>,
    process_diagonal: bool,
}

impl Board {
    fn add_line(&mut self, line: &Line) {
        if line.is_horizontal() {
            for x_index in line.x1..line.x2 + 1 {
                self.board[line.y1][x_index] += 1;
            }
            for x_index in line.x2..line.x1 + 1 {
                self.board[line.y1][x_index] += 1;
            }
        } else if line.is_vertical() {
            for y_index in line.y1..line.y2 + 1 {
                self.board[y_index][line.x1] += 1;
            }
            for y_index in line.y2..line.y1 + 1 {
                self.board[y_index][line.x1] += 1;
            }
        } else if line.is_diagonal() & self.process_diagonal {
            let y_direction: i32 = if line.y1 > line.y2 { -1 } else { 1 };
            let x_direction: i32 = if line.x1 > line.x2 { -1 } else { 1 };

            let slope = line.slope();
            for change in 0..slope + 1 {
                let y_index = (line.y1 as i32 + (change * y_direction)) as usize;
                let x_index = (line.x1 as i32 + (change * x_direction)) as usize;

                self.board[y_index][x_index] += 1;
            }
        }
    }
    fn number_of_points_greater_than(&self, comparison_value: u32) -> u32 {
        let mut count: u32 = 0;
        for y_index in 0..self.board.len() {
            for x_index in 0..self.board[y_index].len() {
                if self.board[y_index][x_index] > comparison_value {
                    count += 1;
                }
            }
        }

        return count;
    }
}

fn main() {
    let mut p1_board = Board {
        board: vec![vec![0; 1000]; 1000],
        process_diagonal: false,
    };

    let mut p2_board = Board {
        board: vec![vec![0; 1000]; 1000],
        process_diagonal: true,
    };

    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let line_definitions: Vec<_> = contents.lines().collect();

    let mut lines: Vec<Line> = Vec::with_capacity(500);

    for line_def in line_definitions {
        let mut line = Line {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        };
        line.from_str(line_def);
        lines.push(line);
    }

    for line in lines {
        p1_board.add_line(&line);
        p2_board.add_line(&line);
    }

    println!("Part 1: {:?}", p1_board.number_of_points_greater_than(1));
    println!("Part 2: {:?}", p2_board.number_of_points_greater_than(1));
}
