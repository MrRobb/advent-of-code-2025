use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> Matrix<char> {
    input.lines().map(|line| line.chars()).collect()
}

fn count_rolls(m: &mut Matrix<char>) -> usize {
    let positions: Vec<_> = m
        .keys()
        .filter(|&pos| m[pos] == '@')
        .filter(|&pos| m.neighbours(pos, true).filter(|&n| m[n] == '@').count() < 4)
        .collect();
    for pos in &positions {
        m[pos] = '.';
    }
    positions.len()
}

pub fn count_rolls_once(input: &str) -> usize {
    let mut m = parse_input(input);
    count_rolls(&mut m)
}

pub fn count_rolls_all(input: &str) -> usize {
    let mut m = parse_input(input);
    let mut total_count = 0;
    loop {
        let count = count_rolls(&mut m);
        total_count += count;
        if count == 0 {
            break;
        }
    }
    total_count
}

pub fn main() {
    let input = std::fs::read_to_string("input/day04.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", count_rolls_once(&input));
    println!("PART 2 = {}", count_rolls_all(&input));
    println!("Execution time: {:?}", now.elapsed());
}
