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

fn part1(coordinates: &[Coordinate]) -> Result<u64> {
    coordinates
        .iter()
        .tuple_combinations()
        .map(|(a, b)| enclosed_area(a, b))
        .max()
        .ok_or_else(|| anyhow!("no max exists, not enough elements given"))
}

fn valid_corners(coordinates: &[Coordinate], a: &Coordinate, b: &Coordinate) -> bool {
    let (min_x, max_x) = (a.x.min(b.x), a.x.max(b.x));
    let (min_y, max_y) = (a.y.min(b.y), a.y.max(b.y));

    // No vertices are strictly within bounding box AND
    !coordinates
        .iter()
        .any(|c| c.x > min_x && c.x < max_x && c.y > min_y && c.y < max_y) &&
    // No edges span the bounding box
    !coordinates
        .iter()
        .circular_tuple_windows()
        .any(|(first, second)| {
            // Note: we know these points share either an x or y in the input
            let same_x = first.x == second.x;
            if same_x {
                let edge_x = first.x;
                // True if a vertical line spanning bounding box
                edge_x > min_x
                    && edge_x < max_x
                    && first.y.min(second.y) <= min_y
                    && first.y.max(second.y) >= max_y
            } else {
                let edge_y = first.y;
                // True if horizontal line spanning bounding box
                edge_y > a.y.min(b.y)
                    && edge_y < a.y.max(b.y)
                    && first.x.min(second.x) <= a.x.min(b.x)
                    && first.x.max(second.x) >= a.x.max(b.x)
            }
        })
}

fn part2(coordinates: &[Coordinate]) -> Result<u64> {
    coordinates
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| valid_corners(coordinates, a, b).then_some(enclosed_area(a, b)))
        .max()
        .ok_or_else(|| anyhow!("no suitable rectangle exists"))
}

fn main() -> Result<()> {
    start_day(DAY);
    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read(INPUT_FILE)?;
    let input = parse_coordinates(BufReader::new(file.as_slice()))?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input)?;
    let p1_elapsed = p1_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_elapsed);

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input)?;
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
    fn parse_coordinates() {
        let result = super::parse_coordinates(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        let input = result.unwrap();
        assert_eq!(8, input.len());
    }

    #[test]
    fn part_1() {
        let expected = 50;
        let input =
            super::parse_coordinates(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 24;
        let input =
            super::parse_coordinates(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(&input);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn large_empty() {}
}
