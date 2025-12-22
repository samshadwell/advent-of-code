use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::time::Instant;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn part1(input: &str) -> Result<i32> {
    input
        .chars()
        .map(|c| match c {
            '(' => Ok(1),
            ')' => Ok(-1),
            _ => Err(anyhow!("unknown character found: {c}")),
        })
        .sum()
}

fn part2(input: &str) -> Result<usize> {
    let mut level = 0;
    for (idx, c) in input.chars().enumerate() {
        level += match c {
            '(' => 1,
            ')' => -1,
            _ => return Err(anyhow!("unknown character found: {c}")),
        };
        if level < 0 {
            return Ok(idx + 1);
        }
    }
    Err(anyhow!("elevator did not reach basement"))
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let input = std::fs::read_to_string(INPUT_FILE)?;
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

    #[test]
    fn part_1() {
        assert_eq!(0, part1("(())").unwrap());
        assert_eq!(0, part1("()()").unwrap());
        assert_eq!(3, part1("(()(()(").unwrap());
        assert_eq!(3, part1("))(((((").unwrap());
        assert_eq!(-1, part1("))(").unwrap());
        assert_eq!(-3, part1(")())())").unwrap());
    }

    #[test]
    fn part_2() {
        assert_eq!(1, part2(")").unwrap());
        assert_eq!(5, part2("()())").unwrap());
    }
}
