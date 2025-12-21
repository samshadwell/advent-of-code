use adv_code_2015::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use md5::Digest;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse<R: BufRead>(reader: &mut R) -> Result<String> {
    let mut buf = String::new();
    let bytes_read = reader.read_line(&mut buf).context("reading line")?;
    if bytes_read <= 1 {
        return Err(anyhow!("failed to read any bytes of input"));
    }
    let trimmed = buf.trim_end().to_owned();
    if trimmed.is_empty() {
        return Err(anyhow!("found empty key"));
    }
    Ok(trimmed)
}

fn find_first_collision(key: &str, max: usize, pred: fn(hash: Digest) -> bool) -> Result<usize> {
    (0..=max)
        .find(|curr| pred(md5::compute(format!("{}{}", key, curr))))
        .ok_or_else(|| anyhow!("no collision found between 0 and {}", max))
}

fn part1(input: &str, max: usize) -> Result<usize> {
    find_first_collision(input, max, |hash| {
        hash[0] == 0 && hash[1] == 0 && hash[2] < 0x10
    })
}

fn part2(input: &str, max: usize) -> Result<usize> {
    find_first_collision(input, max, |hash| {
        hash[0] == 0 && hash[1] == 0 && hash[2] == 0
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
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input, usize::MAX)?;
    println!("Result = {}", result);
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

    // This test takes annoyingly long, but does pass
    #[ignore]
    #[test]
    fn part_1() {
        assert_eq!(609_043, part1("abcdef", 1_000_000).unwrap());
        assert_eq!(1_048_970, part1("pqrstuv", 1_100_000).unwrap());
    }
}
