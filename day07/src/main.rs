use std::fs;

fn calculate_fuel_cost(distance: u32) -> u32 {
    let fuel_cost: u32 = (1..distance + 1).sum();

    return fuel_cost;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut crab_locations: Vec<usize> = contents.split(',').collect::<Vec<_>>().iter().map(|x| usize::from_str_radix(x, 10).expect("Error converting crab locations")).collect();
    crab_locations.sort();

    let mut locations:[u32;2000] = [0; 2000];
    let mut locations_p2:[u32;2000] = [0; 2000];

    for idx in 0..locations.len() {
        for crab_index in 0..crab_locations.len() {
            locations[idx] += ((crab_locations[crab_index] as i32 - idx as i32) as i32).abs() as u32;
            locations_p2[idx] += calculate_fuel_cost(((crab_locations[crab_index] as i32 - idx as i32) as i32).abs() as u32);
        }
    }

    let mut min_place:usize = 0;
    let mut min_place_p2:usize = 0;
    for location_index in 0..locations.len() {
        let location_value = locations[location_index];
        let p2_location_value = locations_p2[location_index];

        if location_value < locations[min_place]{
            min_place = location_index;
        }
        if p2_location_value < locations[min_place_p2]{
            min_place_p2 = location_index;
        }
    }
    println!("Part 1: {}", locations[min_place]);
    println!("Part 2: {}", locations_p2[min_place_p2]);
}
