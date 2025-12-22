use adv_code_2015::start_day;
use anyhow::{Context, Result};
use const_format::concatcp;
use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse<R: BufRead>(reader: R) -> Result<Vec<String>> {
    reader
        .lines()
        .map(|l| l.with_context(|| "error reading line"))
        .collect()
}

fn part1_nice(s: &str) -> bool {
    let num_vowels = s
        .bytes()
        .filter(|&c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
        .count();
    let has_repeats = s.bytes().tuple_windows().any(|(a, b)| a == b);
    let has_forbidden = s.bytes().tuple_windows().any(|tup| {
        matches!(
            tup,
            (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y')
        )
    });

    num_vowels >= 3 && has_repeats && !has_forbidden
}

fn part1<T: AsRef<str>>(input: &[T]) -> usize {
    input.iter().filter(|&s| part1_nice(s.as_ref())).count()
}

fn part2_nice(s: &str) -> bool {
    let bytes = s.as_bytes();
    let has_two_pair = bytes.windows(2).enumerate().any(|(i, pair)| {
        bytes
            .get(i + 2..)
            .is_some_and(|s| s.windows(2).any(|other| pair == other))
    });
    let has_separated_dupe = bytes.iter().tuple_windows().any(|(a, _, b)| a == b);

    has_two_pair && has_separated_dupe
}

fn part2<T: AsRef<str>>(input: &[T]) -> usize {
    input.iter().filter(|&s| part2_nice(s.as_ref())).count()
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
    let result = part1(input.as_slice());
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_nice() {
        // Nice
        assert!(super::part1_nice("ugknbfddgicrmopn"));
        assert!(super::part1_nice("aaa"));

        // Naughty
        assert!(!super::part1_nice("jchzalrnumimnmhp"));
        assert!(!super::part1_nice("haegwjzuvuyypxyu"));
        assert!(!super::part1_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2_nice() {
        // Nice
        assert!(super::part2_nice("qjhvhtzxzqqjkmpb"));
        assert!(super::part2_nice("xxyxx"));

        // Naughty
        assert!(!super::part2_nice("uurcxstgmygtbstg"));
        assert!(!super::part2_nice("ieodomkazucvgmuy"));
    }
}
