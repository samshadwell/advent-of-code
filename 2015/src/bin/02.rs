use adv_code_2015::start_day;
use anyhow::Result;
use anyhow::anyhow;
use const_format::concatcp;
use nom::Finish;
use nom::Parser;
use nom::character::complete::{char, usize};
use nom::sequence::terminated;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

struct Dimensions {
    l: usize,
    w: usize,
    h: usize,
}

impl Dimensions {
    fn required_paper(&self) -> usize {
        let side1 = self.l * self.w;
        let side2 = self.l * self.h;
        let side3 = self.w * self.h;
        let min_size = side1.min(side2).min(side3);
        2 * (side1 + side2 + side3) + min_size
    }

    fn required_ribbon(&self) -> usize {
        let bow_ribbon = self.l * self.w * self.h;
        let mut sorted = [self.l, self.w, self.h];
        sorted.sort();
        let [smallest, second, _] = sorted;

        2 * (smallest + second) + bow_ribbon
    }
}

impl FromStr for Dimensions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (remaining, l) = terminated(usize, char('x'))
            .parse(s)
            .finish()
            .map_err(|_: nom::error::Error<&str>| anyhow!("length parse error"))?;
        let (remaining, w) = terminated(usize, char('x'))
            .parse(remaining)
            .finish()
            .map_err(|_: nom::error::Error<&str>| anyhow!("width parse error"))?;
        let (remaining, h) = usize(remaining)
            .finish()
            .map_err(|_: nom::error::Error<&str>| anyhow!("height parse error"))?;

        if !remaining.is_empty() {
            return Err(anyhow!("trailing characters in input: {}", remaining));
        }

        Ok(Dimensions { l, w, h })
    }
}

fn parse<R: BufRead>(reader: R) -> Result<Vec<Dimensions>> {
    reader.lines().map(|l| Dimensions::from_str(&l?)).collect()
}

fn part1(input: &[Dimensions]) -> usize {
    input.iter().map(Dimensions::required_paper).sum()
}

fn part2(input: &[Dimensions]) -> usize {
    input.iter().map(Dimensions::required_ribbon).sum()
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::File::open(INPUT_FILE)?;
    let input = parse(BufReader::new(file))?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input);
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input);
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
2x3x4
1x1x10
";

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        assert_eq!(2, result.unwrap().len());
    }

    #[test]
    fn part_1() {
        let expected = 58 + 43;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 34 + 14;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(&input);
        assert_eq!(result, expected)
    }
}
