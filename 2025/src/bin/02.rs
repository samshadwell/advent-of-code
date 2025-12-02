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

struct MagicValueIter {
    start: u64,
    end: u64,
    // Invariant: This always points to the next, unreturned, base value at which to start
    // searching for our next magic value. E.g., if 12 then we'd first try number 1212, then
    // 1313, then 1414, and so on, returning only when we find a number between start and end.
    next_base: Option<u64>,
}

impl MagicValueIter {
    fn new(start: u64, end: u64) -> Self {
        let start_str = start.to_string();
        let first_half = &start_str[..(start_str.len() / 2)];
        let next_base = if first_half.is_empty() {
            Some(1)
        } else {
            Some(
                first_half
                    .parse()
                    .expect("parse should succeed by construction"),
            )
        };
        MagicValueIter {
            start,
            end,
            next_base,
        }
    }

    // Check on what the next value would be, ignoring any bounds
    fn peek_candidate_unchecked(&self) -> Option<u64> {
        match self.next_base {
            None => None,
            Some(nb) => {
                let s = format!("{}{}", nb, nb);
                Some(s.parse().expect("parse should succeed by construction"))
            }
        }
    }

    fn increment_candidate(&mut self) {
        match self.next_base {
            None => {}
            Some(nb) => {
                self.next_base = Some(nb + 1);
            }
        }
    }
}

impl Iterator for MagicValueIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_candidate = self.peek_candidate_unchecked();
        if maybe_candidate.is_none() {
            return None;
        }

        loop {
            let candidate = self.peek_candidate_unchecked().unwrap();
            if candidate < self.start {
                // Candidate is too small, increment base, continue loop
                self.increment_candidate();
            } else if candidate >= self.start && candidate <= self.end {
                // Candidate is within range, will be returned outside loop
                break;
            } else {
                // Candidate is too large, there are no valid values
                self.next_base = None;
                break;
            }
        }

        let val = self.peek_candidate_unchecked();
        self.increment_candidate();
        val
    }
}

fn part1<R: BufRead>(reader: R) -> Result<u64> {
    let mut result: u64 = 0;
    for (start, end) in parse_ranges(reader)? {
        for magic_value in MagicValueIter::new(start, end) {
            result += magic_value as u64;
        }
    }
    Ok(result)
}

// TODO: Change result type and implement
fn part2<R: BufRead>(_reader: R) -> Result<u32> {
    Ok(0)
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
        // TODO: Modify expected
        let expected = 0;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
