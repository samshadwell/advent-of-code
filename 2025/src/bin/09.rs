use adv_code_2025::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Instant;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug)]
struct Coordinate {
    x: u64,
    y: u64,
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^(\d+),(\d+)$").expect("valid regex"));

        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid point format: {}", s))?;
        let x = caps[1].parse().context("parsing x")?;
        let y = caps[2].parse().context("parsing y")?;

        Ok(Self { x, y })
    }
}

fn parse_coordinates<R: BufRead>(reader: R) -> Result<Vec<Coordinate>> {
    reader.lines().map(|l| Coordinate::from_str(&l?)).collect()
}

fn enclosed_area(a: &Coordinate, b: &Coordinate) -> u64 {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn part1<R: BufRead>(reader: R) -> Result<u64> {
    let coordinates = parse_coordinates(reader)?;
    coordinates
        .iter()
        .tuple_combinations()
        .map(|(a, b)| enclosed_area(a, b))
        .max()
        .ok_or_else(|| anyhow!("no max exists, not enough elements given"))
}

fn valid_corners(coordinates: &[Coordinate], a: &Coordinate, b: &Coordinate) -> bool {
    // Case 1: There are any vertices strictly within the bounding box. Must be invalid
    if coordinates.iter().any(|c| {
        c.x > a.x.min(b.x) && c.x < a.x.max(b.x) && c.y > a.y.min(b.y) && c.y < a.y.max(b.y)
    }) {
        return false;
    }
    // Case 2: Some line spans the rectangle from outside the bounding box
    for (first, second) in coordinates.iter().circular_tuple_windows() {
        // Note: we know these points share either an x or y
        let same_x = first.x == second.x;
        if same_x {
            // Rules out our candidates if x is within interval and ys are to either side (vertical line)
            let line_x = first.x;
            if line_x > a.x.min(b.x)
                && line_x < a.x.max(b.x)
                && first.y.min(second.y) <= a.y.min(b.y)
                && first.y.max(second.y) >= a.y.max(b.y)
            {
                return false;
            }
        } else {
            // y is within interval and xs are to either side (horizontal line)
            let line_y = first.y;
            if line_y > a.y.min(b.y)
                && line_y < a.y.max(b.y)
                && first.x.min(second.x) <= a.x.min(b.x)
                && first.x.max(second.x) >= a.x.max(b.x)
            {
                return false;
            }
        }
    }
    true
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let coordinates = parse_coordinates(reader)?;
    coordinates
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| valid_corners(&coordinates, a, b).then_some(enclosed_area(a, b)))
        .max()
        .ok_or_else(|| anyhow!("no suitable rectangle exists"))
}

fn main() -> Result<()> {
    let input = std::fs::read(INPUT_FILE)?;
    start_day(DAY);

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part1(input_file)?;
    let p1_elapsed = p1_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_elapsed);

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part2(input_file)?;
    let p2_elapsed = p2_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_elapsed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn part_1() {
        let expected = 50;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 24;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
