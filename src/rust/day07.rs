use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|line| line.chars())).expect("Invalid grid")
}

fn propagate(mut m: Matrix<char>) -> (u64, u64) {
    let original_grid = m.clone();
    let mut splits = 0;
    let mut beams: Vec<u64> = (0..m.columns).map(|c| u64::from(m[(0, c)] == 'S')).collect();
    for i in 1..m.rows {
        let mut next_beams = vec![0; m.columns];
        for j in 0..m.columns {
            let prev = m[(i - 1, j)];
            let current = original_grid[(i, j)];

            match (prev, current) {
                ('|' | 'S', '.') => {
                    m[(i, j)] = '|';
                    next_beams[j] += beams[j];
                },
                ('|', '^') => {
                    splits += 1;
                    m[(i, j - 1)] = '|';
                    m[(i, j + 1)] = '|';
                    next_beams[j - 1] += beams[j];
                    next_beams[j + 1] += beams[j];
                },
                _ => {},
            }
        }
        std::mem::swap(&mut beams, &mut next_beams);
    }
    (splits, beams.iter().sum())
}

pub fn count_splits(input: &str) -> u64 {
    propagate(parse_input(input)).0
}

pub fn count_quantum(input: &str) -> u64 {
    propagate(parse_input(input)).1
}

pub fn main() {
    let input = std::fs::read_to_string("input/day07.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", count_splits(&input));
    println!("PART 2 = {}", count_quantum(&input));
    println!("Execution time: {:?}", now.elapsed());
}
