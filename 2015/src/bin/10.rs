use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse<R: BufRead>(reader: R) -> Result<String> {
    reader
        .lines()
        .next()
        .ok_or_else(|| anyhow!("expected at least one line, found none"))
        .and_then(|l| Ok(l?))
}

fn encode(s: &str) -> String {
    let mut encoded = String::with_capacity(2 * s.len());
    let mut iter = s.chars().peekable();
    while let Some(curr) = iter.next() {
        let mut len = 1;
        while Some(&curr) == iter.peek() {
            len += 1;
            iter.next();
        }
        encoded.push_str(format!("{len}{curr}").as_str());
    }
    encoded
}

fn part1(input: &str) -> usize {
    let mut curr = input.to_string();
    for _ in 0..40 {
        curr = encode(&curr);
    }
    curr.len()
}

fn part2(input: &str) -> usize {
    let mut curr = input.to_string();
    for _ in 0..50 {
        curr = encode(&curr);
    }
    curr.len()
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
    let result = part1(&input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!("11", encode("1"));
        assert_eq!("21", encode("11"));
        assert_eq!("1211", encode("21"));
        assert_eq!("111221", encode("1211"));
        assert_eq!("312211", encode("111221"));
    }
}
