use adv_code_2015::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use md5::Digest;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fmt::Write;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse<R: BufRead>(reader: &mut R) -> Result<String> {
    let mut buf = String::new();
    reader.read_line(&mut buf).context("reading line")?;
    buf.truncate(buf.trim_end().len());
    if buf.is_empty() {
        return Err(anyhow!("found empty key"));
    }
    Ok(buf)
}

fn find_first_collision(
    key: &str,
    max: usize,
    pred: impl Fn(Digest) -> bool + Send + Sync,
) -> Result<usize> {
    let max_digits = format!("{max}").len();

    (0..max)
        .into_par_iter()
        .by_exponential_blocks()
        .map_init(
            || {
                let mut s = String::with_capacity(key.len() + max_digits);
                s.push_str(key);
                s
            },
            |s, i| {
                s.truncate(key.len());
                write!(s, "{i}").expect("write to string always succeeds");
                if pred(md5::compute(&s)) {
                    Some(i)
                } else {
                    None
                }
            },
        )
        .find_first(|&res| res.is_some())
        .flatten()
        .ok_or_else(|| anyhow!("no collision found between 0 and {max}"))
}

fn part1(input: &str, max: usize) -> Result<usize> {
    find_first_collision(input, max, |hash| {
        hash.first() == Some(&0)
            && hash.get(1) == Some(&0)
            && hash.get(2).is_some_and(|&x| x < 0x10)
    })
}

fn part2(input: &str, max: usize) -> Result<usize> {
    find_first_collision(input, max, |hash| {
        hash.first() == Some(&0) && hash.get(1) == Some(&0) && hash.get(2) == Some(&0)
    })
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::File::open(INPUT_FILE)?;
    let input = parse(&mut BufReader::new(file))?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input, usize::MAX)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input, usize::MAX)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
hello-there
";

    #[test]
    fn parse() {
        let result = super::parse(&mut BufReader::new(TEST.as_bytes()));
        assert_eq!("hello-there", result.unwrap())
    }

    #[test]
    fn part_1() {
        assert_eq!(609_043, part1("abcdef", 1_000_000).unwrap());
        assert_eq!(1_048_970, part1("pqrstuv", 1_100_000).unwrap());
    }
}
