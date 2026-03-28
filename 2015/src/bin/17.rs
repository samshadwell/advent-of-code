use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use nom::character::complete::{line_ending, u16};
use nom::multi::separated_list0;
use nom::{Finish, Parser};
use std::cmp::Ordering;
use std::time::Instant;

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse(input: &str) -> Result<Vec<u16>> {
    let (_, values) = separated_list0(line_ending, u16)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error: {e}"))?;
    Ok(values)
}

fn num_solutions(nums: &[u16], start_idx: usize, target: u16) -> usize {
    if target == 0 {
        // If the target is 0, there is exactly one way to get it (take empty set)
        return 1;
    } else if start_idx >= nums.len() {
        return 0;
    }

    let mut total = 0;
    let next_bucket = *nums
        .get(start_idx)
        .expect("guaranteed in-bounds by above check");
    if next_bucket <= target {
        // Can fill bucket i, do so
        total += num_solutions(nums, start_idx + 1, target - next_bucket);
    }
    // Also consider case where we don't use bucket i
    total += num_solutions(nums, start_idx + 1, target);
    total
}

fn part1(input: &[u16], target: u16) -> usize {
    num_solutions(input, 0, target)
}

// Merge the two (ways, num_buckets) tuples, preserving only the number of way with the minimal
// possible buckets
fn merge(a: Option<(usize, usize)>, b: Option<(usize, usize)>) -> Option<(usize, usize)> {
    match (a, b) {
        (None, _) => b,
        (_, None) => a,
        (Some((a_ways, a_buckets)), Some((b_ways, b_buckets))) => match a_buckets.cmp(&b_buckets) {
            Ordering::Less => a,
            Ordering::Greater => b,
            Ordering::Equal => Some((a_ways + b_ways, a_buckets)),
        },
    }
}

// Each tuple (n, m) represents finding n ways to some solution using exactly m buckets
fn ways_to_min_buckets(nums: &[u16], start_idx: usize, target: u16) -> Option<(usize, usize)> {
    if target == 0 {
        // If the target is 0 we get **1** solution by taking **0** buckets
        return Some((1, 0));
    } else if start_idx >= nums.len() {
        return None;
    }

    let mut min_so_far = None;
    let next_bucket = *nums
        .get(start_idx)
        .expect("guaranteed in-bounds by above check");
    if next_bucket <= target {
        // Can fill bucket i, do so
        if let Some((ways, buckets)) =
            ways_to_min_buckets(nums, start_idx + 1, target - next_bucket)
        {
            min_so_far = Some((ways, buckets + 1));
        }
    }
    // Also consider case where we don't use bucket i
    let without_bucket = ways_to_min_buckets(nums, start_idx + 1, target);
    merge(min_so_far, without_bucket)
}

fn part2(input: &[u16], target: u16) -> usize {
    match ways_to_min_buckets(input, 0, target) {
        None => 0,
        Some((ways, _)) => ways,
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read_to_string(INPUT_FILE)?;
    let input = parse(&file)?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input, 150);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input, 150);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
20
15
10
5
5
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        let vals = result.unwrap();
        assert_eq!(vec![20, 15, 10, 5, 5], vals);
    }

    #[test]
    fn part_1() {
        let expected = 4;
        let input = super::parse(TEST).expect("parse succeeds");
        let result = part1(&input, 25);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 3;
        let input = super::parse(TEST).expect("parse succeeds");
        let result = part2(&input, 25);
        assert_eq!(result, expected)
    }
}
