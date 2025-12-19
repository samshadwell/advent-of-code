use adv_code_2015::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "NN"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// TODO: Change type
type Input = i32;

// TODO implement
fn parse<R: BufRead>(_reader: R) -> Result<Input> {
    Ok(0)
}

// TODO: Change result type and implement
fn part1(_input: Input) -> Result<i32> {
    Ok(0)
}

// TODO: Change result type and implement
fn part2(_input: Input) -> Result<i32> {
    Ok(0)
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read(INPUT_FILE)?;
    let input = parse(BufReader::new(file.as_slice()))?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(input)?;
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(input)?;
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok())
    }

    #[test]
    fn part_1() {
        // TODO: Modify expected
        let expected = 0;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(input);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        // TODO: Modify expected
        let expected = 0;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(input);
        assert_eq!(result.unwrap(), expected)
    }
}
