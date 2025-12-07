use adv_code_2025::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::time::Instant;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn find_start(
    lines: &mut impl Iterator<Item = Result<String, std::io::Error>>,
) -> Result<Vec<usize>> {
    Ok(lines
        .next()
        .ok_or_else(|| anyhow!("no lines in input"))??
        .chars()
        .enumerate()
        .filter(|(_, char)| *char == 'S')
        .map(|(idx, _)| idx)
        .collect())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut lines = reader.lines();
    let mut lasers = find_start(&mut lines)?;

    let mut num_splits = 0;
    while let Some(maybe_line) = lines.next() {
        let line = maybe_line?;
        let mut next_lasers: Vec<usize> = Vec::with_capacity(lasers.len());
        for laser in lasers {
            match line.as_bytes().get(laser) {
                None => {
                    return Err(anyhow!(
                        "input malformed, expected lasers to stay within input grid"
                    ));
                }
                Some(b'^') => {
                    num_splits += 1;
                    next_lasers.push(laser - 1);
                    next_lasers.push(laser + 1);
                }
                Some(b'.') => next_lasers.push(laser),
                Some(c) => return Err(anyhow!("input malformed, unexpected character {}", c)),
            }
        }
        next_lasers.dedup();
        lasers = next_lasers;
    }

    Ok(num_splits)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut lines = reader.lines();
    let lasers = find_start(&mut lines)?;
    let mut num_timelines = HashMap::new();
    lasers.iter().for_each(|l| {
        num_timelines.insert(*l, 1usize);
    });

    while let Some(maybe_line) = lines.next() {
        let line = maybe_line?;
        let mut next_timelines = HashMap::new();
        for (idx, timelines) in num_timelines {
            match line.as_bytes().get(idx) {
                None => {
                    return Err(anyhow!(
                        "input malformed, expected lasers to stay within input grid"
                    ));
                }
                Some(b'^') => {
                    *next_timelines.entry(idx - 1).or_insert(0) += timelines;
                    *next_timelines.entry(idx + 1).or_insert(0) += timelines;
                }
                Some(b'.') => *next_timelines.entry(idx).or_insert(0) += timelines,
                Some(c) => return Err(anyhow!("input malformed, unexpected character {}", c)),
            }
        }
        num_timelines = next_timelines;
    }

    Ok(num_timelines.values().sum())
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn part_1() {
        let expected = 21;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 40;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
