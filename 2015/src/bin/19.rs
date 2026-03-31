use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::{count, separated_list0};
use nom::sequence::separated_pair;
use nom::{Finish, Parser};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::time::Instant;

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

type Replacements<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> Result<(Replacements<'_>, &str)> {
    let single_rule = separated_pair(alpha1, tag(" => "), alpha1);
    let rules = separated_list0(line_ending, single_rule);
    let molecule = alpha1;

    let (_, (rules, molecule)) = separated_pair(rules, count(line_ending, 2), molecule)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error: {e}"))?;

    let grouped_replacements = rules.iter().map(|(k, v)| (k, *v)).chunk_by(|(k, _)| *k);
    let mut replacements = HashMap::new();
    for (k, tups) in &grouped_replacements {
        replacements.insert(*k, tups.map(|tup| tup.1).collect());
    }

    Ok((replacements, molecule))
}

fn part1(replacements: &Replacements, molecule: &str) -> Result<usize> {
    let mut set = HashSet::new();
    let max_replacement_input = replacements
        .keys()
        .map(|k| k.len())
        .max()
        .unwrap_or_default();

    for window_len in 0..=max_replacement_input {
        for window_start in 0..molecule.len() {
            let (before, rest) = molecule
                .split_at_checked(window_start)
                .ok_or_else(|| anyhow!("string contains non-ascii characters"))?;
            match rest.split_at_checked(window_len) {
                None => {}
                Some((to_replace, tail)) => match replacements.get(to_replace) {
                    None => {}
                    Some(rs) => {
                        for r in rs {
                            let mut buf = String::new();
                            write!(buf, "{before}{r}{tail}")?;
                            set.insert(buf);
                        }
                    }
                },
            }
        }
    }

    Ok(set.len())
}

// TODO: Change result type and implement
#[allow(clippy::missing_const_for_fn)]
fn part2(_input: i32) -> i32 {
    0
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read_to_string(INPUT_FILE)?;
    let (replacements, molecule) = parse(&file)?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&replacements, molecule)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(0);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
H => HO
H => OH
O => HH

HOH
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        let (replacements, molecule) = result.unwrap();
        assert_eq!(
            HashMap::from([("H", vec!["HO", "OH"]), ("O", vec!["HH"])]),
            replacements
        );
        assert_eq!("HOH", molecule);
    }

    #[test]
    fn part_1() {
        let (replacements, molecule) = super::parse(TEST).expect("parse succeeds");
        assert_eq!(4, part1(&replacements, molecule).expect("succeeds"));
        assert_eq!(7, part1(&replacements, "HOHOHO").expect("succeeds"))
    }

    #[test]
    fn part_2() {
        // TODO: Modify expected
        let expected = 0;
        let _input = super::parse(TEST).expect("parse succeeds");
        let result = part2(0);
        assert_eq!(result, expected)
    }
}
