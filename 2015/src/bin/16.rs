use adv_code_2015::start_day;
use anyhow::Result;
use anyhow::anyhow;
use const_format::concatcp;
use nom::Finish;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u16};
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::fmt::Display;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

struct MfcsamSample {
    children: u16,
    cats: u16,
    samoyeds: u16,
    pomeranians: u16,
    akitas: u16,
    vizslas: u16,
    goldfish: u16,
    trees: u16,
    cars: u16,
    perfumes: u16,
}

impl MfcsamSample {
    fn matches_p1(&self, partial: &AuntMemory) -> bool {
        (partial.children.is_none_or(|v| v == self.children))
            && (partial.cats.is_none_or(|v| v == self.cats))
            && (partial.samoyeds.is_none_or(|v| v == self.samoyeds))
            && (partial.pomeranians.is_none_or(|v| v == self.pomeranians))
            && (partial.akitas.is_none_or(|v| v == self.akitas))
            && (partial.vizslas.is_none_or(|v| v == self.vizslas))
            && (partial.goldfish.is_none_or(|v| v == self.goldfish))
            && (partial.trees.is_none_or(|v| v == self.trees))
            && (partial.cars.is_none_or(|v| v == self.cars))
            && (partial.perfumes.is_none_or(|v| v == self.perfumes))
    }

    fn matches_p2(&self, partial: &AuntMemory) -> bool {
        (partial.children.is_none_or(|v| v == self.children))
            && (partial.cats.is_none_or(|v| v > self.cats))
            && (partial.samoyeds.is_none_or(|v| v == self.samoyeds))
            && (partial.pomeranians.is_none_or(|v| v < self.pomeranians))
            && (partial.akitas.is_none_or(|v| v == self.akitas))
            && (partial.vizslas.is_none_or(|v| v == self.vizslas))
            && (partial.goldfish.is_none_or(|v| v < self.goldfish))
            && (partial.trees.is_none_or(|v| v > self.trees))
            && (partial.cars.is_none_or(|v| v == self.cars))
            && (partial.perfumes.is_none_or(|v| v == self.perfumes))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct AuntMemory {
    id: u16,
    children: Option<u16>,
    cats: Option<u16>,
    samoyeds: Option<u16>,
    pomeranians: Option<u16>,
    akitas: Option<u16>,
    vizslas: Option<u16>,
    goldfish: Option<u16>,
    trees: Option<u16>,
    cars: Option<u16>,
    perfumes: Option<u16>,
}

impl AuntMemory {
    fn new(id: u16) -> Self {
        AuntMemory {
            id,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
}

#[derive(Debug)]
enum UnknownAttrError<I> {
    Nom(I, nom::error::ErrorKind),
    UnknownAttr(String),
}

impl<I> nom::error::ParseError<I> for UnknownAttrError<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        UnknownAttrError::Nom(input, kind)
    }
    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
impl<I: Display> Display for UnknownAttrError<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nom(input, kind) => write!(f, "nom error ({kind:?}) at: {input}"),
            Self::UnknownAttr(attr) => write!(f, "unknown attribute: {attr}"),
        }
    }
}

fn parse_sue(s: &str) -> IResult<&str, AuntMemory, UnknownAttrError<&str>> {
    let (res, (_, id, _, attrs)) = all_consuming((
        tag("Sue "),
        u16,
        tag(": "),
        separated_list1(tag(", "), separated_pair(alpha1, tag(": "), u16)),
    ))
    .parse(s)?;

    let mut s = AuntMemory::new(id);
    for (k, v) in attrs {
        let field = match k {
            "children" => &mut s.children,
            "cats" => &mut s.cats,
            "samoyeds" => &mut s.samoyeds,
            "pomeranians" => &mut s.pomeranians,
            "akitas" => &mut s.akitas,
            "vizslas" => &mut s.vizslas,
            "goldfish" => &mut s.goldfish,
            "trees" => &mut s.trees,
            "cars" => &mut s.cars,
            "perfumes" => &mut s.perfumes,
            _ => {
                return Err(nom::Err::Failure(UnknownAttrError::UnknownAttr(
                    k.to_string(),
                )));
            }
        };
        *field = Some(v);
    }
    Ok((res, s))
}

fn parse<R: BufRead>(reader: R) -> Result<Vec<AuntMemory>> {
    reader
        .lines()
        .map(|l| {
            let line = l.map_err(|e| anyhow!("error reading line {e}"))?;
            let res = parse_sue(&line)
                .finish()
                .map_err(|e| anyhow!("parse error {e}"))?;
            Ok(res.1)
        })
        .collect()
}

fn solve<F>(aunts: &[AuntMemory], key: &MfcsamSample, f: F) -> Result<u16>
where
    F: Fn(&MfcsamSample, &AuntMemory) -> bool,
{
    let matches: Vec<_> = aunts.iter().filter(|a| f(key, a)).collect();
    if matches.is_empty() {
        Err(anyhow!("No matches found for given key"))
    } else if matches.len() > 1 {
        Err(anyhow!("More than one match found for given key"))
    } else {
        Ok(matches.first().unwrap().id)
    }
}

fn part1(aunts: &[AuntMemory], key: &MfcsamSample) -> Result<u16> {
    solve(aunts, key, |k, a| k.matches_p1(a))
}

fn part2(aunts: &[AuntMemory], key: &MfcsamSample) -> Result<u16> {
    solve(aunts, key, |k, a| k.matches_p2(a))
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::File::open(INPUT_FILE)?;
    let sues = parse(BufReader::new(file))?;
    let analysis = MfcsamSample {
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&sues, &analysis)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&sues, &analysis)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
Sue 1: children: 1, cars: 8, vizslas: 7
";

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        let sues = result.unwrap();
        assert_eq!(1, sues.len());
        assert_eq!(
            sues.first().unwrap(),
            &AuntMemory {
                id: 1,
                children: Some(1),
                cars: Some(8),
                vizslas: Some(7),
                cats: None,
                samoyeds: None,
                pomeranians: None,
                akitas: None,
                goldfish: None,
                trees: None,
                perfumes: None,
            }
        );
    }
}
