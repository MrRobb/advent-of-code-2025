
enum Direction {
  L(u64),
  R(u64)
}

fn parse(input: &str) -> Vec<Direction> {
  input.lines().map(|line| {
    match line.chars().next().unwrap() {
      'L' => Direction::L(line[1..].parse::<u64>().unwrap()),
      'R' => Direction::R(line[1..].parse::<u64>().unwrap()),
      _ => panic!("Invalid direction"),
    }
  }).collect()
}

fn zero_ticks_end(input: Vec<Direction>) -> u64 {
    let mut location = 50;
    let mut zero_ticks = 0;
    for direction in input {
        location = match direction {
          Direction::L(amount) => (location - amount).rem_euclid(100),
          Direction::R(amount) => (location + amount).rem_euclid(100),
        };
        if location == 0 {
            zero_ticks += 1;
        }
    }
    zero_ticks
}

fn zero_ticks_any(input: Vec<Direction>) -> u64 {
    let mut location = 50;
    let mut zero_ticks = 0;
    for direction in input {
      match direction {
        Direction::L(amount) => {
          for _ in 0..amount {
            location = (location - 1).rem_euclid(100);
            if location == 0 {
                zero_ticks += 1;
            }
          }
        },
        Direction::R(amount) => {
          for _ in 0..amount {
            location = (location + 1).rem_euclid(100);
            if location == 0 {
                zero_ticks += 1;
            }
          }
        },
      }
    }
    zero_ticks
}

pub fn main() {
    let input = std::fs::read_to_string("input/day01.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", zero_ticks_end(&input));
    println!("PART 2 = {}", zero_ticks_any(&input));
    println!("Execution time: {:?}", now.elapsed());
}