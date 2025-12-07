use adv_code_2025::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn max_joltage(bank: &str, num_digits: usize) -> Result<u128> {
    if !bank.chars().all(|c| c.is_ascii_digit()) {
        return Err(anyhow!("expected digit-only string, got: {}", bank));
    }

    if num_digits == 0 {
        return Err(anyhow!("require num_digits to be at least 1, got 0"));
    }

    if bank.len() < num_digits {
        return Err(anyhow!(
            "invalid number digits in battery bank. Required at least {}, got {}",
            num_digits,
            bank.len()
        ));
    }

    let bytes = bank.as_bytes();
    let mut digits = Vec::with_capacity(num_digits);
    let mut start_idx = 0;

    for i in 0..num_digits {
        let remaining_to_find = num_digits - i;
        let end_idx = bytes.len() - remaining_to_find;

        let (max_offset, max_digit) = bytes
            .get(start_idx..=end_idx)
            .unwrap_or(&[])
            .iter()
            .enumerate()
            // using negated min_by_key to get the _first_ largest digit (max_by_key would return last)
            .min_by_key(|&(_, d)| -(*d as i32))
            .unwrap();

        digits.push(*max_digit);
        start_idx += max_offset + 1;
    }

    if digits.len() != num_digits {
        unreachable!(
            "programmer error, should get exactly num_digits digits, got {}",
            digits.len()
        )
    }

    Ok(digits
        .iter()
        .fold(0, |acc, &d| acc * 10 + (d - b'0') as u128))
}

fn solve<R: BufRead>(reader: R, num_digits: usize) -> Result<u128> {
    reader.lines().map(|l| max_joltage(&l?, num_digits)).sum()
}

fn part1<R: BufRead>(reader: R) -> Result<u128> {
    solve(reader, 2)
}

fn part2<R: BufRead>(reader: R) -> Result<u128> {
    solve(reader, 12)
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
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn part_1() {
        let expected = 357;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 3121910778619;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
