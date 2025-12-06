enum Operation {
    Multiply,
    Add,
}

impl Operation {
    fn from_str(op: &str) -> Self {
        match op {
            "*" => Self::Multiply,
            "+" => Self::Add,
            _ => panic!("Invalid operation"),
        }
    }
}

fn parse_input_columns(input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let cols = input
        .lines()
        .take_while(|l| !l.starts_with(['*', '+']))
        .flat_map(|line| line.split_whitespace().enumerate())
        .fold(Vec::new(), |mut acc, (i, num_str)| {
            if i >= acc.len() {
                acc.resize_with(i + 1, Vec::new);
            }
            acc[i].push(num_str.parse().unwrap());
            acc
        });
    (cols, parse_operations(input))
}

fn parse_input_right_to_left(input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let cols = input
        .lines()
        .take_while(|l| !l.starts_with(['*', '+']))
        .flat_map(|line| line.chars().enumerate())
        .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, u64::from(d))))
        // Extract numbers column-wise
        // Columns -> [col1_num, col2_num, None, col3_num, ...]
        .fold(Vec::<Option<u64>>::new(), |mut acc, (i, d)| {
            if i >= acc.len() {
                acc.resize(i + 1, None);
            }
            match &mut acc[i] {
                Some(num) => *num = *num * 10 + d,
                None => acc[i] = Some(d),
            }
            acc
        })
        .into_iter()
        // Using None as a separator between groups of numbers
        .fold(vec![vec![]], |mut groups, opt_num| {
            match opt_num {
                Some(num) => groups.last_mut().unwrap().push(num),
                None => groups.push(Vec::new()),
            }
            groups
        });
    (cols, parse_operations(input))
}

fn parse_operations(input: &str) -> Vec<Operation> {
    input
        .lines()
        .last()
        .expect("Input should have at least one line")
        .split_whitespace()
        .map(Operation::from_str)
        .collect()
}

fn calculate(numbers: &[Vec<u64>], operations: &[Operation]) -> u64 {
    operations
        .iter()
        .zip(numbers.iter())
        .map(|(op, nums)| match op {
            Operation::Multiply => nums.iter().product::<u64>(),
            Operation::Add => nums.iter().sum(),
        })
        .sum()
}

pub fn calculate_columns(input: &str) -> u64 {
    let (numbers, operations) = parse_input_columns(input);
    calculate(&numbers, &operations)
}

pub fn calculate_right_to_left(input: &str) -> u64 {
    let (numbers, operations) = parse_input_right_to_left(input);
    calculate(&numbers, &operations)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day06.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", calculate_columns(&input));
    println!("PART 2 = {}", calculate_right_to_left(&input));
    println!("Execution time: {:?}", now.elapsed());
}
