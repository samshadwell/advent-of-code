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
        self.common_matches(partial)
            && partial.cats.is_none_or(|v| v == self.cats)
            && partial.pomeranians.is_none_or(|v| v == self.pomeranians)
            && partial.goldfish.is_none_or(|v| v == self.goldfish)
            && partial.trees.is_none_or(|v| v == self.trees)
    }

    fn matches_p2(&self, partial: &AuntMemory) -> bool {
        self.common_matches(partial)
            && partial.cats.is_none_or(|v| v > self.cats)
            && partial.pomeranians.is_none_or(|v| v < self.pomeranians)
            && partial.goldfish.is_none_or(|v| v < self.goldfish)
            && partial.trees.is_none_or(|v| v > self.trees)
    }

    fn common_matches(&self, partial: &AuntMemory) -> bool {
        partial.children.is_none_or(|v| v == self.children)
            && partial.samoyeds.is_none_or(|v| v == self.samoyeds)
            && partial.akitas.is_none_or(|v| v == self.akitas)
            && partial.vizslas.is_none_or(|v| v == self.vizslas)
            && partial.cars.is_none_or(|v| v == self.cars)
            && partial.perfumes.is_none_or(|v| v == self.perfumes)
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
    const fn new(id: u16) -> Self {
        Self {
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
        Self::Nom(input, kind)
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
            let (_, aunt) = parse_sue(&line)
                .finish()
                .map_err(|e| anyhow!("parse error on line '{line}': {e}"))?;
            Ok(aunt)
        })
        .collect()
}

fn solve<F>(aunts: &[AuntMemory], key: &MfcsamSample, f: F) -> Result<u16>
where
    F: Fn(&MfcsamSample, &AuntMemory) -> bool,
{
    let mut matches = aunts.iter().filter(|a| f(key, a));
    matches.next().map_or_else(
        || Err(anyhow!("No matches found")),
        |first| {
            if matches.next().is_some() {
                Err(anyhow!("More than one match found"))
            } else {
                Ok(first.id)
            }
        },
    )
}

fn part1(aunts: &[AuntMemory], key: &MfcsamSample) -> Result<u16> {
    solve(aunts, key, MfcsamSample::matches_p1)
}

fn part2(aunts: &[AuntMemory], key: &MfcsamSample) -> Result<u16> {
    solve(aunts, key, MfcsamSample::matches_p2)
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
Sue 2: akitas: 10, perfumes: 10, children: 5
Sue 3: cars: 5, pomeranians: 4, vizslas: 1
Sue 4: cats: 10, trees: 10, pomeranians: 0, goldfish: 0
";

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        let sues = result.unwrap();
        assert_eq!(4, sues.len());
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

    #[test]
    fn part1() {
        let aunts = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let key = MfcsamSample {
            children: 5,
            cats: 100,
            samoyeds: 100,
            pomeranians: 100,
            akitas: 10,
            vizslas: 100,
            goldfish: 100,
            trees: 100,
            cars: 100,
            perfumes: 10,
        };

        let result = super::part1(&aunts, &key);
        assert!(result.is_ok());
        assert_eq!(2, result.unwrap());
    }

    #[test]
    fn part2() {
        let aunts = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let key = MfcsamSample {
            children: 5,
            cats: 5,
            samoyeds: 5,
            pomeranians: 5,
            akitas: 5,
            vizslas: 5,
            goldfish: 5,
            trees: 5,
            cars: 5,
            perfumes: 5,
        };

        let result = super::part2(&aunts, &key);
        assert!(result.is_ok());
        assert_eq!(4, result.unwrap());
    }
}
