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

    let replacements = rules.into_iter().into_group_map();
    Ok((replacements, molecule))
}

fn next_molecules(replacements: &Replacements, molecule: &str) -> Result<HashSet<String>> {
    let mut set = HashSet::new();
    for (&from, tos) in replacements {
        for (start, _) in molecule.match_indices(from) {
            let end = start + from.len();
            for &to in tos {
                let mut buf = String::with_capacity(molecule.len() + to.len() - from.len());
                write!(buf, "{}{to}{}", &molecule[..start], &molecule[end..])?;
                set.insert(buf);
            }
        }
    }

    Ok(set)
}

fn part1(replacements: &Replacements, molecule: &str) -> Result<usize> {
    Ok(next_molecules(replacements, molecule)?.len())
}

fn part2(replacements: &Replacements, target: &str) -> Result<usize> {
    // This one is tricky. First I tried doing something like BFS, but it blows up way too quickly.
    // So the second solution (which works on my input) is to instead do it greedily, working backwards.
    // Take all the X => Y replacements, at every step replace the longest such Y that appears in our
    // string with its corresponding X. Repeat this until we get the initial "e" string, or we can't
    // do any more steps.
    // This happens to work for my input but it certainly is not a general solution that would work for
    // all possible replacements/goal strings. It probably works for all the AoC inputs, though, due to
    // some cleverness in the problem formulation.
    let mut inverted: Vec<_> = replacements
        .iter()
        .flat_map(|(&k, vs)| vs.iter().map(move |&v| (v, k)))
        .collect();
    inverted.sort_by_key(|(v, _)| std::cmp::Reverse(v.len()));

    let mut curr = target.to_string();
    let mut num_replacements = 0;
    while curr != "e" {
        let mut did_replacement = false;
        for (long, short) in &inverted {
            if curr.contains(long) {
                curr = curr.replacen(long, short, 1);
                num_replacements += 1;
                did_replacement = true;
                break;
            }
        }
        if !did_replacement {
            return Err(anyhow!("did not find way create given target"));
        }
    }

    Ok(num_replacements)
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
    let result = part2(&replacements, molecule)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
e => H
e => O
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
            HashMap::from([
                ("e", vec!["H", "O"]),
                ("H", vec!["HO", "OH"]),
                ("O", vec!["HH"])
            ]),
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
        let (replacements, _) = super::parse(TEST).expect("parse succeeds");
        assert_eq!(3, part2(&replacements, "HOH").expect("succeeds"));
        assert_eq!(6, part2(&replacements, "HOHOHO").expect("succeeds"));
    }
}
