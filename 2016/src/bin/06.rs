use adv_code_2016::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use nom::{
    Finish, Parser,
    character::complete::{alpha1, newline},
    multi::separated_list0,
};
use std::{collections::HashMap, time::Instant};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse(input: &str) -> Result<Vec<&str>> {
    let (_, strings) = separated_list0(newline, alpha1)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;
    Ok(strings)
}

fn solve<F>(input: &[&str], select: F) -> String
where
    F: Fn(&HashMap<char, i32>) -> char,
{
    let mut char_iters = input.iter().map(|s| s.chars()).collect_vec();
    let mut result = String::new();
    loop {
        let mut counts = HashMap::new();
        for iter in &mut char_iters {
            match iter.next() {
                None => {
                    return result;
                }
                Some(c) => {
                    counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }
        let c = select(&counts);
        result.push(c);
    }
}

fn part1(input: &[&str]) -> String {
    solve(input, |counts| {
        *counts
            .iter()
            .max_by_key(|(_, v)| **v)
            .map(|(k, _)| k)
            .expect("will exist")
    })
}

fn part2(input: &[&str]) -> String {
    solve(input, |counts| {
        *counts
            .iter()
            .min_by_key(|(_, v)| **v)
            .map(|(k, _)| k)
            .expect("will exist")
    })
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read_to_string(INPUT_FILE)?;
    let input = parse(&file)?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input);
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
    use super::*;

    const TEST: &str = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        let v = result.unwrap();
        assert_eq!(v.len(), 16);
        assert_eq!(
            v[0..5],
            vec!["eedadn", "drvtee", "eandsr", "raavrd", "atevrs"]
        );
    }

    #[test]
    fn part_1() {
        let input = super::parse(TEST).expect("parse succeeds");
        assert_eq!(part1(&input), "easter")
    }

    #[test]
    fn part_2() {
        let input = super::parse(TEST).expect("parse succeeds");
        assert_eq!(part2(&input), "advent")
    }
}
