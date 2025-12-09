use itertools::Itertools;
use petgraph::graph::UnGraph;
use petgraph::{algo::tarjan_scc, graph::NodeIndex};
use std::collections::HashSet;

type Point = [i64; 3];

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn join_graph(points: &[Point], max_joints: Option<usize>) -> (UnGraph<usize, ()>, Option<(usize, usize)>) {
    let mut edges = (0..points.len())
        .tuple_combinations()
        .map(|(i, j)| {
            let (p1, p2) = (points[i], points[j]);
            let dist =
                ((p1[0] - p2[0]).pow(2) as f64 + (p1[1] - p2[1]).pow(2) as f64 + (p1[2] - p2[2]).pow(2) as f64).sqrt();
            (i, j, dist)
        })
        .collect::<Vec<_>>();

    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut graph = UnGraph::<usize, ()>::new_undirected();
    for i in 0..points.len() {
        graph.add_node(i);
    }

    let mut in_graph = HashSet::new();
    let mut last_joints = None;
    for (i, j, _) in edges.into_iter().take(max_joints.unwrap_or(usize::MAX)) {
        if !in_graph.contains(&i) || !in_graph.contains(&j) {
            last_joints = Some((i, j));
        }
        in_graph.insert(i);
        in_graph.insert(j);
        graph.add_edge(NodeIndex::new(i), NodeIndex::new(j), ());
    }

    (graph, last_joints)
}

pub fn count_connected_components(input: &str) -> usize {
    let points = parse_input(input);
    let (joint_graph, _) = join_graph(&points, Some(1000));
    tarjan_scc(&joint_graph)
        .into_iter()
        .map(|comp| comp.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn count_last_joint(input: &str) -> i64 {
    let points = parse_input(input);
    if let (_, Some((u, v))) = join_graph(&points, None) {
        points[u][0] * points[v][0]
    } else {
        unreachable!()
    }
}

pub fn main() {
    let input = std::fs::read_to_string("input/day08.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", count_connected_components(&input));
    println!("PART 2 = {}", count_last_joint(&input));
    println!("Execution time: {:?}", now.elapsed());
}
