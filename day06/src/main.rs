use std::fs;

struct LanternFish {
    spawn_timer: u8,
}

fn main() {
    let mut fish_list: Vec<LanternFish> = vec![];

    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let fish_timers: Vec<_> = contents.split(',').collect();

    let mut fish_counts: [usize; 9] = [0; 9];

    for fish in fish_timers {
        let spawn_timer = usize::from_str_radix(fish, 10).expect("Error converting val") as u8;
        fish_counts[spawn_timer as usize] += 1;

        let new_fish = LanternFish { spawn_timer };

        fish_list.push(new_fish);
    }

    for day in 0..80 {
        println!("Day: {:?}. Fish Count: {:?}", day, fish_list.len());
        for i in 0..fish_list.len() {
            if fish_list[i].spawn_timer == 0 {
                fish_list.push(LanternFish { spawn_timer: 8 });
                fish_list[i].spawn_timer = 6;
            } else {
                fish_list[i].spawn_timer -= 1;
            }
        }
    }

    println!("Part 1: {} fish", fish_list.len());

    let mut count_sum: usize = 0;
    for day in 0..256 {
        fish_counts[(day + 7) % 9] += fish_counts[day % 9];
        count_sum = fish_counts.iter().sum();
        println!("Day: {:?}. Fish Count: {:?}", day + 1, count_sum);
    }

    println!("Part 2: {} fish", count_sum);
}
