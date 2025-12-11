use cached::proc_macro::cached;
use std::{collections::HashMap, convert::Into};

type Graph = HashMap<String, Vec<String>>;

fn parse_input(input: &str) -> Graph {
    input
        .lines()
        .map(|line| {
            let (node, neighbors) = line.split_once(':').unwrap();
            let neighbors = neighbors.split_whitespace().map(Into::into).collect();
            (node.to_string(), neighbors)
        })
        .collect()
}

#[cached(
    key = "(String, bool, bool, String)",
    convert = r#"{ (node.to_string(), dac, fft, end.to_string()) }"#
)]
fn dfs(graph: &Graph, node: &str, mut dac: bool, mut fft: bool, end: &str) -> usize {
    if node == end {
        return (dac && fft).into();
    }
    if node == "dac" {
        dac = true;
    }
    if node == "fft" {
        fft = true;
    }
    graph
        .get(node)
        .unwrap()
        .iter()
        .map(|neighbor| dfs(graph, neighbor, dac, fft, end))
        .sum()
}

pub fn find_all_paths(input: &str) -> usize {
    let graph = parse_input(input);
    dfs(&graph, "you", true, true, "out")
}

pub fn find_all_paths_with_dac_and_fft(input: &str) -> usize {
    let graph = parse_input(input);
    dfs(&graph, "svr", false, false, "out")
}

pub fn main() {
    let input = std::fs::read_to_string("input/day11.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", find_all_paths(&input));
    println!("PART 2 = {}", find_all_paths_with_dac_and_fft(&input));
    println!("Execution time: {:?}", now.elapsed());
}
