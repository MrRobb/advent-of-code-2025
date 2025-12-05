use std::cmp::Ordering;
use std::ops::Range;

use itertools::Itertools;

type Ingredient = usize;
type IngredientRange = Range<Ingredient>;
type IngredientRanges = Vec<IngredientRange>;

fn parse_input(input: &str) -> (IngredientRanges, Vec<Ingredient>) {
    let (ingredient_ranges, ingredient_list) = input.split_once("\n\n").unwrap();
    let ingredient_ranges = ingredient_ranges
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start = start_str.parse::<Ingredient>().unwrap();
            let end = end_str.parse::<Ingredient>().unwrap() + 1;
            start..end
        })
        .collect();
    let ingredient_list = ingredient_list
        .lines()
        .map(|line| line.parse::<Ingredient>().unwrap())
        .collect();
    (ingredient_ranges, ingredient_list)
}

fn merge_ranges(ranges: IngredientRanges) -> IngredientRanges {
    ranges
        .into_iter()
        .sorted_by_key(|range| range.start)
        .coalesce(|prev, next| {
            if prev.end >= next.start {
                Ok(prev.start..prev.end.max(next.end))
            } else {
                Err((prev, next))
            }
        })
        .collect()
}

fn is_in_range(range: &IngredientRange, ingredient: Ingredient) -> Ordering {
    if range.contains(&ingredient) {
        Ordering::Equal
    } else if range.start > ingredient {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

pub fn how_many_fresh_check(input: &str) -> usize {
    let (ingredient_ranges, ingredient_list) = parse_input(input);
    let merged_ranges = merge_ranges(ingredient_ranges);
    ingredient_list
        .into_iter()
        .filter(|&ingredient| {
            merged_ranges
                .binary_search_by(|range| is_in_range(range, ingredient))
                .is_ok()
        })
        .count()
}

pub fn how_many_fresh_all(input: &str) -> usize {
    let (ingredient_ranges, _) = parse_input(input);
    merge_ranges(ingredient_ranges)
        .into_iter()
        .map(|range| range.len())
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day05.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", how_many_fresh_check(&input));
    println!("PART 2 = {}", how_many_fresh_all(&input));
    println!("Execution time: {:?}", now.elapsed());
}
