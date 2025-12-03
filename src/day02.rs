use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|s| {
            let ns = s.split_once('-').unwrap();
            (ns.0.parse::<u64>().unwrap(), ns.1.parse::<u64>().unwrap())
        })
        .collect()
}

fn is_invalid_two(n: u64) -> bool {
    let cs = n.to_string().chars().collect::<Vec<char>>();
    if cs.len() % 2 != 0 {
        return false;
    }
    cs[0..cs.len() / 2] == cs[cs.len() / 2..]
}

fn is_invalid_any(n: u64) -> bool {
    let cs = n.to_string().chars().collect::<Vec<char>>();
    (1..=cs.len() / 2)
        .filter(|&l| cs.len() % l == 0)
        .any(|length| cs.chunks(length).all_equal())
}

pub fn find_invalid_two(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|(start, end)| (start..=end).filter(|&n| is_invalid_two(n)).sum::<u64>())
        .sum()
}

pub fn find_invalid_any(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .map(|(start, end)| (start..=end).filter(|&n| is_invalid_any(n)).sum::<u64>())
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day02.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", find_invalid_two(&input));
    println!("PART 2 = {}", find_invalid_any(&input));
    println!("Execution time: {:?}", now.elapsed());
}
