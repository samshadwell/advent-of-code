use adv_code_2025::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse_ranges<R: BufRead>(reader: R) -> Result<Vec<(u64, u64)>> {
    let mut result = Vec::new();
    for split in reader.split(b',') {
        let range: Vec<u8> = split?
            .iter()
            .copied()
            .filter(|&byte| byte == b'-' || byte.is_ascii_digit())
            .collect();

        let mut parts = range.split(|&byte| byte == b'-');
        let start_bytes = parts
            .next()
            .ok_or_else(|| anyhow!("range should contain exactly two parts, found zero"))?;
        let end_bytes = parts
            .next()
            .ok_or_else(|| anyhow!("range should contain exactly two parts, found one"))?;
        if parts.next().is_some() {
            return Err(anyhow!(
                "range should contain exactly two parts, found at least three"
            ));
        }

        let start_str = std::str::from_utf8(start_bytes)?;
        let end_str = std::str::from_utf8(end_bytes)?;

        let start = start_str
            .parse()
            .with_context(|| format!("failed to parse start interval string: {}", start_str))?;
        let end = end_str
            .parse()
            .with_context(|| format!("failed to parse end interval string: {}", end_str))?;
        result.push((start, end));
    }
    Ok(result)
}

// true if the number consists of a sequence of digits which repeats at least twice and at most max_repeats times
// when expressed as base-10. E.g.:
// - 11 is '1' repeated twice
// - 123123 is '123' repeated twice
// - 4444 is both '44' repeated twice '4' repeated four times
// - 123123123 is '123' repeated three times
fn is_repeating_number(num: u64, max_repeats: usize) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();
    'outer: for repeats in 2..=max_repeats {
        if repeats > bytes.len() {
            break;
        }
        if !bytes.len().is_multiple_of(repeats) {
            continue;
        };

        let subseq_len = bytes.len() / repeats;
        for i in subseq_len..bytes.len() {
            if bytes[i] != bytes[i % subseq_len] {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

fn solve<R: BufRead>(reader: R, max_repeats: usize) -> Result<u64> {
    let ranges = parse_ranges(reader)?;
    let sum = ranges
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        .filter(|&v| is_repeating_number(v, max_repeats))
        .sum();
    Ok(sum)
}

fn part1<R: BufRead>(reader: R) -> Result<u64> {
    solve(reader, 2)
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    solve(reader, usize::MAX)
}

fn main() -> Result<()> {
    let input = std::fs::read(INPUT_FILE)?;
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part1(input_file)?;
    let p1_elapsed = p1_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_elapsed);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part2(input_file)?;
    let p2_elapsed = p2_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_elapsed);
    //endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

    #[test]
    fn part_1() {
        let expected = 1227775554;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 4174379265;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
