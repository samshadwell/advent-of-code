use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32};
use nom::combinator::all_consuming;
use nom::{Finish, IResult, Parser};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

type Input = HashMap<(String, String), i32>;

fn parse_line(s: &str) -> IResult<&str, (&str, &str, i32)> {
    all_consuming((
        alpha1,
        tag(" would "),
        alt((tag("gain "), tag("lose "))),
        i32,
        tag(" happiness units by sitting next to "),
        alpha1,
        tag("."),
    ))
    .parse(s)
    .map(|(res, (a, _, dir, points, _, b, _))| match dir {
        "gain " => (res, (a, b, points)),
        "lose " => (res, (a, b, -points)),
        _ => unreachable!("alt should only match above literals"),
    })
}

fn parse<R: BufRead>(reader: R) -> Result<Input> {
    let mut result = HashMap::new();
    for res_l in reader.lines() {
        let line = res_l.map_err(|e| anyhow!("error reading line {e}"))?;
        let (_, (a, b, points)) = parse_line(line.as_str())
            .finish()
            .map_err(|e| anyhow!("parse error {e}"))?;
        result.insert((a.to_owned(), b.to_owned()), points);
    }

    Ok(result)
}

fn all_guests(input: &Input) -> Vec<&str> {
    input.keys().map(|(a, _)| a.as_str()).unique().collect()
}

fn part1(input: &Input) -> Result<i32> {
    let guests = all_guests(input);
    let max = guests
        .iter()
        .permutations(guests.len())
        .map(|perm| {
            perm.iter()
                .circular_tuple_windows()
                .map(|(&&a, &&b)| -> Result<i32> {
                    let ab = input
                        .get(&(a.to_string(), b.to_string()))
                        .ok_or_else(|| anyhow!("missing edge {a} -> {b}"))?;
                    let ba = input
                        .get(&(b.to_string(), a.to_string()))
                        .ok_or_else(|| anyhow!("missing edge {b} -> {a}"))?;
                    Ok(ab + ba)
                })
                .sum::<Result<i32>>()
        })
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .max()
        .unwrap_or_default();

    Ok(max)
}

fn part2(input: &Input) -> Result<i32> {
    let guests = all_guests(input);
    let max = guests
        .iter()
        .permutations(guests.len())
        .map(|perm| {
            perm.iter()
                // Note: This is the only difference between these two functions.
                // I simulate seating myself by not evaluating the last <> first
                // guests, implicitly seating myself between the first and last guests
                .tuple_windows()
                .map(|(&&a, &&b)| -> Result<i32> {
                    let ab = input
                        .get(&(a.to_string(), b.to_string()))
                        .ok_or_else(|| anyhow!("missing edge {a} -> {b}"))?;
                    let ba = input
                        .get(&(b.to_string(), a.to_string()))
                        .ok_or_else(|| anyhow!("missing edge {b} -> {a}"))?;
                    Ok(ab + ba)
                })
                .sum::<Result<i32>>()
        })
        .collect::<Result<Vec<i32>>>()?
        .into_iter()
        .max()
        .unwrap_or_default();

    Ok(max)
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::File::open(INPUT_FILE)?;
    let input = parse(BufReader::new(file))?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(12, parsed.len());

        assert_eq!(
            Some(-79),
            parsed
                .get(&("Alice".to_string(), "Carol".to_string()))
                .copied()
        );
        assert_eq!(
            Some(55),
            parsed
                .get(&("Carol".to_string(), "David".to_string()))
                .copied()
        );
    }

    #[test]
    fn part_1() {
        let expected = 330;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input).expect("part1 succeeds");
        assert_eq!(result, expected)
    }
}
