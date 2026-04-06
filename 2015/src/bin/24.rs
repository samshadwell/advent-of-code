use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use nom::character::complete::{line_ending, usize as nom_usize};
use nom::multi::separated_list0;
use nom::{Finish, Parser};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

const DAY: &str = "24";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse(input: &str) -> Result<Vec<usize>> {
    let (_, packages) = separated_list0(line_ending, nom_usize)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;
    Ok(packages)
}

fn better_solution(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    match a.len().cmp(&b.len()) {
        Ordering::Less => a,
        Ordering::Greater => b,
        Ordering::Equal => {
            let qe_a = a.iter().product::<usize>();
            let qe_b = b.iter().product();
            match qe_a.cmp(&qe_b) {
                Ordering::Less | Ordering::Equal => a,
                Ordering::Greater => b,
            }
        }
    }
}

fn solve<'a>(
    target: usize,
    packages: &'a [usize],
    memo: &mut HashMap<(usize, &'a [usize]), Vec<usize>>,
) -> Result<Vec<usize>> {
    if target == 0 {
        return Ok(vec![]);
    }

    if let Some(v) = memo.get(&(target, packages)) {
        return Ok(v.clone());
    }

    let best = match packages.split_first() {
        None => Err(anyhow!("did not find way to make {target}")),
        Some((p, rest)) if *p > target => solve(target, rest, memo),
        Some((p, rest)) => {
            // Implicit else: p <= target
            let with_p = solve(target - p, rest, memo).map(|mut v| {
                v.push(*p);
                v
            });
            let without_p = solve(target, rest, memo);
            match (with_p, without_p) {
                (Err(_), Err(_)) => Err(anyhow!("did not find way to make {target}")),
                (a, Err(_)) => a,
                (Err(_), b) => b,
                (Ok(sol_a), Ok(sol_b)) => Ok(better_solution(sol_a, sol_b)),
            }
        }
    }?;

    memo.insert((target, packages), best.clone());
    Ok(best)
}

fn part1(input: &[usize]) -> Result<usize> {
    let total = input.iter().sum::<usize>();
    if !total.is_multiple_of(3) {
        return Err(anyhow!(
            "given packages cannot be divided into three even groups"
        ));
    }

    let group_total = total / 3;
    let solution = solve(group_total, input, &mut HashMap::new())?;
    Ok(solution.iter().product())
}

fn part2(input: &[usize]) -> Result<usize> {
    let total = input.iter().sum::<usize>();
    if !total.is_multiple_of(4) {
        return Err(anyhow!(
            "given packages cannot be divided into four even groups"
        ));
    }

    let group_total = total / 4;
    let solution = solve(group_total, input, &mut HashMap::new())?;
    Ok(solution.iter().product())
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
    let result = part1(&input)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
1
2
3
4
5
7
8
9
10
11
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        let v = result.unwrap();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn part_1() {
        let expected = 99;
        let input = super::parse(TEST).expect("parse succeeds");
        let result = part1(&input).expect("succeeds");
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 44;
        let input = super::parse(TEST).expect("parse succeeds");
        let result = part2(&input).expect("succeeds");
        assert_eq!(result, expected)
    }
}
