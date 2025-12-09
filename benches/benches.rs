use std::fs::read_to_string;

#[allow(clippy::wildcard_imports)]
use advent_of_code_2025::*;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench1(c: &mut Criterion) {
    // let input01 = day01::parse(&read_to_string("input/day01.txt").expect("Input file not found"));
    // c.bench_function("Day 1 | Part 1", |b| b.iter(|| day01::zero_ticks_end(&input01)));
    // c.bench_function("Day 1 | Part 2", |b| b.iter(|| day01::zero_ticks_any(&input01)));

    // let input02 = read_to_string("input/day02.txt").expect("Input file not found");
    // c.bench_function("Day 2 | Part 1", |b| b.iter(|| day02::find_invalid_two(&input02)));
    // c.bench_function("Day 2 | Part 2", |b| b.iter(|| day02::find_invalid_any(&input02)));

    // let input03 = read_to_string("input/day03.txt").expect("Input file not found");
    // c.bench_function("Day 3 | Part 1", |b| b.iter(|| day03::find_batteries_2(&input03)));
    // c.bench_function("Day 3 | Part 2", |b| b.iter(|| day03::find_batteries_12(&input03)));

    // let input04 = read_to_string("input/day04.txt").expect("Input file not found");
    // c.bench_function("Day 4 | Part 1", |b| b.iter(|| day04::count_rolls_once(&input04)));
    // c.bench_function("Day 4 | Part 2", |b| b.iter(|| day04::count_rolls_all(&input04)));

    // let input05 = read_to_string("input/day05.txt").expect("Input file not found");
    // c.bench_function("Day 5 | Part 1", |b| b.iter(|| day05::how_many_fresh_check(&input05)));
    // c.bench_function("Day 5 | Part 2", |b| b.iter(|| day05::how_many_fresh_all(&input05)));

    // let input06 = read_to_string("input/day06.txt").expect("Input file not found");
    // c.bench_function("Day 6 | Part 1", |b| b.iter(|| day06::calculate_columns(&input06)));
    // c.bench_function("Day 6 | Part 2", |b| {
    //     b.iter(|| day06::calculate_right_to_left(&input06));
    // });

    // let input07 = read_to_string("input/day07.txt").expect("Input file not found");
    // c.bench_function("Day 7 | Part 1", |b| b.iter(|| day07::count_splits(&input07)));
    // c.bench_function("Day 7 | Part 2", |b| {
    //     b.iter(|| day07::count_quantum(&input07));
    // });

    let input08 = read_to_string("input/day08.txt").expect("Input file not found");
    c.bench_function("Day 8 | Part 1", |b| {
        b.iter(|| day08::count_connected_components(&input08))
    });
    c.bench_function("Day 8 | Part 2", |b| {
        b.iter(|| day08::count_last_joint(&input08));
    });
}

criterion_group!(benches, bench1);
criterion_main!(benches);
