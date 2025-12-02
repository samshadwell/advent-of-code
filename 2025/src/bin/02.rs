use adv_code_2025::*;
use anyhow::*;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let start_str = std::str::from_utf8(
            parts
                .next()
                .expect("range should contain exactly two parts, found zero"),
        )?;
        let end_str = std::str::from_utf8(
            parts
                .next()
                .expect("range should contain exactly two parts, found one"),
        )?;
        if parts.next().is_some() {
            return Err(anyhow!(
                "range should contain exactly two parts, found at least three"
            ));
        }

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

// true if the number consists of a sequence of digits which repeats twice when expressed
// as base-10. E.g., 11, 123123, or 4444 would all return true
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

fn part1<R: BufRead>(reader: R) -> Result<u64> {
    let mut result: u64 = 0;
    for (start, end) in parse_ranges(reader)? {
        for v in start..=end {
            if is_repeating_number(v, 2) {
                result += v;
            }
        }
    }
    Ok(result)
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let mut result: u64 = 0;
    for (start, end) in parse_ranges(reader)? {
        for v in start..=end {
            if is_repeating_number(v, usize::MAX) {
                result += v;
            }
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
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
