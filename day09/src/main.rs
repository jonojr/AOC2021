use std::collections::HashMap;
use std::fs;

struct Basin<'a> {
    low_point: (usize, usize),
    points_to_explore: Vec<(usize, usize)>,
    points: HashMap<(usize, usize), bool>,
    y_max: usize,
    x_max: usize,
    heightmap: &'a Vec<Vec<u32>>,
}

impl Basin<'_> {
    fn get_surrounding_points(&self, current: (usize, usize)) -> Vec<(usize, usize)> {
        let mut results: Vec<(usize, usize)> = vec![];
        if current.0 > 0 {
            results.push((current.0 - 1, current.1));
        }
        if current.0 < self.y_max {
            results.push((current.0 + 1, current.1));
        }

        if current.1 > 0 {
            results.push((current.0, current.1 - 1));
        }
        if current.1 < self.x_max {
            results.push((current.0, current.1 + 1));
        }

        return results;
    }

    fn next_iteration(&mut self) {
        for _ in 0..self.points_to_explore.len() {
            let current_point = self.points_to_explore.pop().unwrap();
            let current_point_value = self.heightmap[current_point.0][current_point.1];

            for point in self.get_surrounding_points(current_point) {
                let point_height = self.heightmap[point.0][point.1];
                if point_height != 9
                    && point_height > current_point_value
                    && !self.points.contains_key(&point)
                {
                    self.points_to_explore.push(point);
                    self.points.insert(point, true);
                }
            }
        }
    }

    fn fill_basin(&mut self) -> usize {
        while self.points_to_explore.len() > 0 {
            self.next_iteration();
        }
        return self.points.len() + 1;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let heightmap_strs: Vec<_> = contents.lines().collect();

    let mut heightmap: Vec<Vec<u32>> = vec![];

    for row in heightmap_strs {
        let conv_row: Vec<u32> = row.chars().map(|x| x.to_digit(10).unwrap()).collect();
        heightmap.push(conv_row);
    }

    let mut low_point_heights: Vec<u32> = vec![];
    let mut low_points: Vec<(usize, usize)> = vec![];

    let y_len: usize = heightmap.len();
    let x_len: usize = heightmap[0].len();

    for y_idx in 0..y_len {
        for x_idx in 0..x_len {
            let current_height = heightmap[y_idx][x_idx];
            let mut all_higher = true;

            if y_idx > 0 {
                all_higher &= heightmap[y_idx - 1][x_idx] > current_height
            }
            if y_idx < y_len - 1 {
                all_higher &= heightmap[y_idx + 1][x_idx] > current_height
            }

            if x_idx > 0 {
                all_higher &= heightmap[y_idx][x_idx - 1] > current_height
            }
            if x_idx < x_len - 1 {
                all_higher &= heightmap[y_idx][x_idx + 1] > current_height
            }

            if all_higher {
                low_points.push((y_idx, x_idx));
                low_point_heights.push(current_height);
            }
        }
    }

    println!(
        "Part 1: {:?}",
        low_point_heights.iter().map(|x| x + 1).sum::<u32>()
    );

    let mut basins: Vec<Basin> = vec![];

    for low_point in low_points {
        let temp_basin = Basin {
            low_point,
            points_to_explore: vec![low_point],
            points: HashMap::new(),
            y_max: y_len - 1,
            x_max: x_len - 1,
            heightmap: &heightmap,
        };

        basins.push(temp_basin);
    }

    let mut basin_sizes: Vec<usize> = vec![];

    for mut basin in basins {
        basin_sizes.push(basin.fill_basin());
    }

    basin_sizes.sort();

    println!(
        "Part 2: {:?}",
        basin_sizes[basin_sizes.len() - 3..]
            .iter()
            .product::<usize>()
    );
}
