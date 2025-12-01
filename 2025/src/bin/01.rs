use adv_code_2025::*;
use anyhow::*;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse(line: &str) -> Result<i32> {
    let dir = line.chars().next();
    let sign = match dir {
        Some('L') => -1,
        Some('R') => 1,
        _ => return Err(anyhow!("Unknown direction {:?}", dir)),
    };
    let distance = line[1..].parse::<i32>()?;
    Ok(sign * distance)
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut count = 0;
    let mut current = 50;
    for res in reader.lines() {
        let line = res?;
        if line.is_empty() {
            continue;
        }
        let delta = parse(&line)?;
        current = (current + delta).rem_euclid(100);
        if current == 0 {
            count += 1;
        }
    }
    Ok(count)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut count = 0;
    let mut current = 50;
    for res in reader.lines() {
        let line = res?;
        if line.is_empty() {
            continue;
        }
        let delta = parse(&line)?;
        let before_mod = current + delta;
        let incr = match before_mod {
            // For negative position ends: If we started on 0 and went left, we _did not_ necessarily pass 0
            // In all other cases, we started positive and ended negative, implying passing zero at least once
            ..0 => (if current == 0 { 0 } else { 1 }) + (-before_mod / 100) as usize,
            0 => 1,
            1.. => (before_mod / 100) as usize,
        };
        count += incr;
        current = before_mod.rem_euclid(100);
    }
    Ok(count)
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn part_1() {
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn part_2() {
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), 6);
    }
}
