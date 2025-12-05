use adv_code_2025::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use regex::Regex;
use std::cmp::{Ordering, max};
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;
use std::slice;
use std::sync::LazyLock;
use std::time::Instant;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

type Id = u64;
type Interval = (Id, Id);
#[derive(Debug)]
pub struct MergedSortedIntervals(Vec<Interval>);

fn parse<R: BufRead>(reader: R) -> Result<(Vec<Interval>, Vec<Id>)> {
    let mut parsing_intervals = true;
    let mut intervals = Vec::new();
    let mut ids = Vec::new();

    for line in reader.lines() {
        let s = line?;
        if s.is_empty() {
            parsing_intervals = false;
            continue;
        }

        if parsing_intervals {
            intervals.push(parse_interval(&s)?);
        } else {
            ids.push(s.parse()?);
        }
    }

    Ok((intervals, ids))
}

fn parse_interval(s: &String) -> Result<Interval> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(\d+)-(\d+)$").expect("valid regex"));
    let caps = RE
        .captures(s)
        .ok_or_else(|| anyhow!("invalid interval format: {}", s))?;

    let start = caps[1].parse().context("parsing start")?;
    let end = caps[2].parse().context("parsing end")?;

    Ok((start, end))
}

fn merge_intervals(mut intervals: Vec<Interval>) -> MergedSortedIntervals {
    if intervals.is_empty() {
        return MergedSortedIntervals(intervals);
    }

    intervals.sort_by_key(|(s, _)| *s);
    let mut merged = Vec::with_capacity(intervals.len());
    merged.push(*intervals.first().expect("intervals nonempty"));
    for (next_s, next_e) in intervals {
        let (_, last_e) = merged.last_mut().expect("merged nonempty");
        match next_s.cmp(last_e) {
            Ordering::Less | Ordering::Equal => *last_e = max(*last_e, next_e),
            Ordering::Greater => merged.push((next_s, next_e)),
        }
    }

    MergedSortedIntervals(merged)
}

impl MergedSortedIntervals {
    fn contains(&self, id: Id) -> bool {
        self.0
            .binary_search_by(|(s, e)| {
                if id >= *s && id <= *e {
                    Ordering::Equal
                } else if id < *s {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .is_ok()
    }

    fn iter(&self) -> slice::Iter<'_, Interval> {
        self.0.iter()
    }
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let (intervals, ids) = parse(reader)?;
    let merged = merge_intervals(intervals);
    Ok(ids.iter().filter(|id| merged.contains(**id)).count())
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let (intervals, _) = parse(reader)?;
    Ok(merge_intervals(intervals)
        .iter()
        .map(|(s, e)| e - s + 1)
        .sum())
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn part_1() {
        let expected = 3;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 14;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
