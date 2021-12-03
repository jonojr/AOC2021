use std::fs;

const LOW_BIT_MASK: u32 = 0b00000000000000000000000000000001;
const TWELVE_BIT_MASK: u32 = 0b00000000000000000000111111111111;

fn get_bit_at_index(value: u32, index: u8) -> u32 {
    // Index 0 = highest bit, 1 = lowest bit
    let shifted_value = value >> 31 - index;
    return shifted_value & LOW_BIT_MASK;
}

fn get_most_common_bit_at_index(values: &Vec<u32>, index: u8) -> u32 {
    let mut zeros: u32 = 0;
    let mut ones: u32 = 0;

    for value in values {
        let bit = get_bit_at_index(*value, index);
        if bit == 0 {
            zeros += 1;
        } else {
            ones += 1;
        }
    }

    return if zeros > ones { 0 } else { 1 };
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let values: Vec<_> = contents.lines().collect();

    let mut oxy_gen_values: Vec<u32> = Vec::with_capacity(1000);
    let mut c02_scrubber_values: Vec<u32> = Vec::with_capacity(1000);

    for val in values {
        oxy_gen_values.push(usize::from_str_radix(val, 2).expect("Error converting val") as u32);
        c02_scrubber_values
            .push(usize::from_str_radix(val, 2).expect("Error converting val") as u32);
    }

    let mut gamma_rate: u32 = get_most_common_bit_at_index(&oxy_gen_values, 20);

    for i in 21..32 {
        gamma_rate = gamma_rate << 1;
        gamma_rate += get_most_common_bit_at_index(&oxy_gen_values, i);
    }
    let epsilon_rate = (!gamma_rate) & TWELVE_BIT_MASK;

    println!("Gamma Rate: {}", gamma_rate);
    println!("Epsilon Rate: {}", epsilon_rate);

    println!("Part 1: {}", gamma_rate * epsilon_rate);

    // Part 2

    let mut bit_index: u8 = 32 - 12;
    while oxy_gen_values.len() > 1 && bit_index < 32 {
        let most_common_bit = get_most_common_bit_at_index(&oxy_gen_values, bit_index);
        oxy_gen_values = oxy_gen_values
            .iter()
            .filter(|&&val| get_bit_at_index(val, bit_index) == most_common_bit)
            .copied()
            .collect::<Vec<_>>();

        bit_index += 1;
    }

    println!("Oxygen Generator: {}", oxy_gen_values[0]);

    bit_index = 32 - 12;
    while c02_scrubber_values.len() > 1 && bit_index < 32 {
        let least_common_bit =
            !get_most_common_bit_at_index(&c02_scrubber_values, bit_index) & LOW_BIT_MASK;

        c02_scrubber_values = c02_scrubber_values
            .iter()
            .filter(|&&val| get_bit_at_index(val, bit_index) == least_common_bit)
            .copied()
            .collect::<Vec<_>>();

        bit_index += 1;
    }

    println!("C02 Scrubber: {}", c02_scrubber_values[0]);

    println!("Part 2: {}", oxy_gen_values[0] * c02_scrubber_values[0]);
}
