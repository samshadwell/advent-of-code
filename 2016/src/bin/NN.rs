use adv_code_2016::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::time::Instant;

const DAY: &str = "NN"; // TODO
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// TODO: Remove Clippy allows
#[allow(
    clippy::unnecessary_wraps,
    clippy::trivially_copy_pass_by_ref,
    clippy::missing_const_for_fn
)]
fn parse(_input: &str) -> Result<usize> {
    Ok(0)
}

// TODO: Remove Clippy allows
#[allow(
    clippy::unnecessary_wraps,
    clippy::trivially_copy_pass_by_ref,
    clippy::missing_const_for_fn
)]
fn part1(_input: &usize) -> Result<usize> {
    Ok(0)
}

// TODO: Remove Clippy allows
#[allow(
    clippy::unnecessary_wraps,
    clippy::trivially_copy_pass_by_ref,
    clippy::missing_const_for_fn
)]
fn part2(_input: &usize) -> Result<usize> {
    Ok(0)
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

    // TODO: Override
    const TEST: &str = "\
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
    }

    #[test]
    fn part_1() {
        let expected = 0;
        let input = super::parse(TEST).expect("parse succeeds");
        let result = part1(&input).expect("succeeds");
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 0;
        let input = super::parse(TEST).expect("parse succeeds");
        let result = part2(&input).expect("succeeds");
        assert_eq!(result, expected)
    }
}
