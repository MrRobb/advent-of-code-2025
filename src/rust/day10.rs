use itertools::Itertools;
use z3::{Optimize, SatResult, ast::Int};

struct Machine {
    lights: Vec<i64>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    let parse_vec = |s: &str| {
        s[1..s.len() - 1]
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<i64>>()
    };

    input
        .lines()
        .map(|line| {
            let (lights_str, rest) = line.split_once(' ').unwrap();
            let mut parts = rest.split(' ').collect::<Vec<_>>();
            let joltage = parse_vec(parts.pop().unwrap());

            Machine {
                lights: lights_str[1..lights_str.len() - 1]
                    .chars()
                    .map(|c| (c == '#').into())
                    .collect(),
                buttons: parts
                    .iter()
                    .map(|p| parse_vec(p).into_iter().flat_map(usize::try_from).collect())
                    .collect(),
                joltage,
            }
        })
        .collect()
}

fn solve_indicator_lights(machine: &Machine, use_joltage: bool) -> i64 {
    let solver = Optimize::new();
    let zero = Int::from_i64(0);

    let presses = (0..machine.buttons.len())
        .map(|i| {
            let v = Int::new_const(format!("button_{i}"));
            // P[i] > 0
            solver.assert(&v.ge(&zero));
            v
        })
        .collect_vec();

    let targets = if use_joltage { &machine.joltage } else { &machine.lights };

    targets.iter().enumerate().for_each(|(i, &target)| {
        let effective_button_presses = machine
            .buttons
            .iter()
            .zip(presses.iter())
            // Only select P[i] where button affects component i
            .filter(|(b, _)| b.contains(&i))
            .map(|(_, p)| p)
            .collect::<Vec<_>>();
        let equation = if use_joltage {
            // sum(P[i]) = target
            Int::add(&effective_button_presses).eq(Int::from_i64(target))
        } else {
            // Can only be 0 or 1
            // sum(P[i]) % 2 = target
            Int::add(&effective_button_presses)
                .modulo(Int::from_i64(2))
                .eq(Int::from_i64(target))
        };
        solver.assert(&equation);
    });

    // min(sum(P[i]))
    let total_presses = Int::add(&presses);
    solver.minimize(&total_presses);

    if solver.check(&[]) == SatResult::Sat {
        let model = solver.get_model().unwrap();
        model.eval(&total_presses, true).unwrap().as_i64().unwrap()
    } else {
        0
    }
}

pub fn solve_all_indicator_lights(input: &str) -> i64 {
    let machines = parse_input(input);
    machines.iter().map(|m| solve_indicator_lights(m, false)).sum()
}

pub fn solve_all_joltage_requirements(input: &str) -> i64 {
    let machines = parse_input(input);
    machines.iter().map(|m| solve_indicator_lights(m, true)).sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day10.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", solve_all_indicator_lights(&input));
    println!("PART 2 = {}", solve_all_joltage_requirements(&input));
    println!("Execution time: {:?}", now.elapsed());
}
