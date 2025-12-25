use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::fmt::Write;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse<R: BufRead>(reader: R) -> Result<String> {
    reader
        .lines()
        .next()
        .ok_or_else(|| anyhow!("expected at least one line, found none"))?
        .map_err(anyhow::Error::from)
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
        write!(&mut encoded, "{len}{curr}").expect("write to string always succeeds");
    }
    encoded
}

fn repeated_encoding_length(input: &str, k: usize) -> usize {
    let mut curr = input.to_string();
    for _ in 0..k {
        curr = encode(&curr);
    }
    curr.len()
}

fn part1(input: &str) -> usize {
    repeated_encoding_length(input, 40)
}

fn part2(input: &str) -> usize {
    repeated_encoding_length(input, 50)
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

    #[test]
    fn test_repeated_encoding_length() {
        assert_eq!(2, repeated_encoding_length("1", 1));
        assert_eq!(6, repeated_encoding_length("1", 5));
    }
}
