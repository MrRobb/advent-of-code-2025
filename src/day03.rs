use std::cmp::Reverse;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn find_batteries(input: &[u32], digits: usize) -> u64 {
    let mut positions = vec![0; digits];
    let length = input.len();
    for digit in 0..digits {
        // Calculate the range to search for the maximum digit
        let start_range = if digit == 0 { 0 } else { positions[digit - 1] + 1 };
        let end_range = length - (digits - digit);

        // Find the position of the maximum digit in the specified range
        let position = input[start_range..=end_range]
            .iter()
            .enumerate()
            .max_by_key(|&(a, b)| (b, Reverse(a)))
            .unwrap();

        // Store the absolute position
        positions[digit] = position.0 + start_range;
    }

    // Calculate the resulting number
    positions
        .into_iter()
        .enumerate()
        .map(|(i, pos)| u64::from(input[pos]) * 10u64.pow(u32::try_from(digits - 1 - i).unwrap()))
        .sum()
}

pub fn find_batteries_2(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|line| find_batteries(&line, 2))
        .sum()
}

pub fn find_batteries_12(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|line| find_batteries(&line, 12))
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day03.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", find_batteries_2(&input));
    println!("PART 2 = {}", find_batteries_12(&input));
    println!("Execution time: {:?}", now.elapsed());
}
