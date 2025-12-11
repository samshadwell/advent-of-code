use adv_code_2025::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use nom::character::complete::{alpha1, char, multispace0};
use nom::multi::many0;
use nom::sequence::{delimited, terminated};
use nom::{Finish, IResult, Parser};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader};
use std::ops;
use std::time::Instant;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

type Graph<'a> = HashMap<String, HashSet<String>>;

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, node) = terminated(alpha1, char(':')).parse(input)?;
    let (input, neighbors) = many0(delimited(multispace0, alpha1, multispace0)).parse(input)?;
    Ok((input, (node, neighbors)))
}

fn parse<R: BufRead>(reader: R) -> Result<Graph<'static>> {
    let mut g = HashMap::new();
    for line in reader.lines() {
        let l = line?;
        let (_, (node, neighbors)) = parse_line
            .parse(&l)
            .finish()
            .map_err(|e| anyhow!("Parse error: {}", e))?;
        let node_owned = node.to_string();
        let adj = g.entry(node_owned).or_insert_with(|| HashSet::new());
        adj.extend(neighbors.iter().map(|n| n.to_string()));
        for nbr in neighbors {
            g.entry(nbr.to_string()).or_insert_with(|| HashSet::new());
        }
    }
    Ok(g)
}

fn part1(g: &Graph) -> usize {
    let mut num_ways: HashMap<&str, usize> = HashMap::new();
    let mut queue: VecDeque<_> = VecDeque::new();

    let start_label = "you";
    num_ways.insert(start_label, 1usize);
    queue.push_back(start_label);
    while let Some(node) = queue.pop_front() {
        let n = *num_ways.get(node).unwrap_or(&0);
        if n == 0 {
            continue;
        }
        let neighbors = g.get(node);
        if neighbors.is_none() || neighbors.is_some_and(|nbrs| nbrs.is_empty()) {
            continue;
        }
        for nbr in neighbors.unwrap() {
            *num_ways.entry(nbr).or_insert(0) += n;
            queue.push_back(nbr);
        }
        num_ways.insert(node, 0);
    }

    *num_ways.get("out").unwrap_or(&0)
}

#[derive(Debug, Clone, Copy)]
// One insight: Rather than storing exactly which of "dac" and "fft" each path has seen,
// we can just store how many. If there are no cycles, then in valid paths seeing two
// means we've seen both.
struct P2Ways {
    pass_neither: usize,
    pass_one: usize,
    pass_both: usize,
}

impl P2Ways {
    fn zero() -> Self {
        Self {
            pass_neither: 0,
            pass_one: 0,
            pass_both: 0,
        }
    }

    fn one() -> Self {
        Self {
            pass_neither: 1,
            pass_one: 0,
            pass_both: 0,
        }
    }

    fn pass_special(&mut self) {
        *self = Self {
            pass_neither: 0,
            pass_one: self.pass_neither,
            pass_both: self.pass_one + self.pass_both,
        }
    }

    fn is_zero(&self) -> bool {
        self.pass_both == 0 && self.pass_one == 0 && self.pass_neither == 0
    }
}

impl ops::AddAssign for P2Ways {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            pass_neither: self.pass_neither + rhs.pass_neither,
            pass_one: self.pass_one + rhs.pass_one,
            pass_both: self.pass_both + rhs.pass_both,
        }
    }
}

fn part2(g: &Graph) -> usize {
    let mut num_ways: HashMap<&str, P2Ways> = HashMap::new();
    let mut queue: VecDeque<_> = VecDeque::new();

    let start_label = "svr";
    num_ways.insert(start_label, P2Ways::one());
    queue.push_back(start_label);
    while let Some(node) = queue.pop_front() {
        let mut n = num_ways.get(node).map(|n| *n).unwrap_or(P2Ways::zero());
        if n.is_zero() {
            continue;
        }

        if node == "dac" || node == "fft" {
            n.pass_special();
        }

        let neighbors = g.get(node);
        if neighbors.is_none() || neighbors.is_some_and(|nbrs| nbrs.is_empty()) {
            continue;
        }
        for nbr in neighbors.unwrap() {
            *num_ways.entry(nbr).or_insert(P2Ways::zero()) += n;
            queue.push_back(nbr);
        }
        num_ways.insert(node, P2Ways::zero());
    }

    num_ways.get("out").unwrap_or(&P2Ways::zero()).pass_both
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read(INPUT_FILE)?;
    let input = parse(BufReader::new(file.as_slice()))?;
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
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const TEST_PART_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
    #[test]
    fn parse() {
        assert!(super::parse(BufReader::new(TEST.as_bytes())).is_ok());
        assert!(super::parse(BufReader::new(TEST_PART_2.as_bytes())).is_ok());
    }

    #[test]
    fn part_1() {
        let expected = 5;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 2;
        let input = super::parse(BufReader::new(TEST_PART_2.as_bytes())).expect("parse succeeds");
        let result = part2(&input);
        assert_eq!(result, expected)
    }
}
