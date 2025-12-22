use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use nom::branch::alt;
use nom::bytes::tag;
use nom::character::complete::{char, usize};
use nom::combinator::{all_consuming, map, verify};
use nom::sequence::separated_pair;
use nom::{Finish, IResult, Parser};
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    kind: Kind,
    c1: Coordinate,
    c2: Coordinate,
}

#[derive(Debug, PartialEq, Eq)]
enum Kind {
    TurnOn,
    Toggle,
    TurnOff,
}

#[derive(Debug, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    map(
        verify(separated_pair(usize, char(','), usize), |(x, y)| {
            *x <= 999 && *y <= 999
        }),
        |(x, y)| Coordinate { x, y },
    )
    .parse(input)
}

fn coord_range_parser(input: &str) -> IResult<&str, (Coordinate, Coordinate)> {
    separated_pair(parse_coordinate, tag(" through "), parse_coordinate).parse(input)
}

fn parse_instruction(input: &str) -> Result<Instruction> {
    all_consuming(map(
        (
            alt((
                tag("turn on ").map(|_| Kind::TurnOn),
                tag("turn off ").map(|_| Kind::TurnOff),
                tag("toggle ").map(|_| Kind::Toggle),
            )),
            coord_range_parser,
        ),
        |(kind, (c1, c2))| Instruction { kind, c1, c2 },
    ))
    .parse(input)
    .finish()
    .map(|(_, i)| i)
    .map_err(|e| anyhow!("parse error: {}", e))
}

fn parse<R: BufRead>(reader: R) -> Result<Vec<Instruction>> {
    reader.lines().map(|l| parse_instruction(&l?)).collect()
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];
    for i in instructions {
        // Matching outside the loop lets us get ~30x better perf than matching inside
        match i.kind {
            Kind::TurnOn => {
                for row in &mut lights[i.c1.x..=i.c2.x] {
                    row[i.c1.y..=i.c2.y].fill(true);
                }
            }
            Kind::TurnOff => {
                for row in &mut lights[i.c1.x..=i.c2.x] {
                    row[i.c1.y..=i.c2.y].fill(false);
                }
            }
            Kind::Toggle => {
                for row in &mut lights[i.c1.x..=i.c2.x] {
                    for light in &mut row[i.c1.y..=i.c2.y] {
                        *light = !*light;
                    }
                }
            }
        }
    }
    lights
        .iter()
        .map(|r| r.iter().filter(|&&b| b).count())
        .sum()
}

fn part2(instructions: &[Instruction]) -> u64 {
    let mut lights = vec![vec![0u16; 1000]; 1000];
    for i in instructions {
        // Like above, matching outside the loop lets us get ~30x better perf
        match i.kind {
            Kind::TurnOn => {
                for row in &mut lights[i.c1.x..=i.c2.x] {
                    for light in &mut row[i.c1.y..=i.c2.y] {
                        *light += 1;
                    }
                }
            }
            Kind::TurnOff => {
                for row in &mut lights[i.c1.x..=i.c2.x] {
                    for light in &mut row[i.c1.y..=i.c2.y] {
                        *light = light.saturating_sub(1);
                    }
                }
            }
            Kind::Toggle => {
                for row in &mut lights[i.c1.x..=i.c2.x] {
                    for light in &mut row[i.c1.y..=i.c2.y] {
                        *light += 2;
                    }
                }
            }
        }
    }
    lights
        .iter()
        .map(|r| r.iter().map(|&v| v as u64).sum::<u64>())
        .sum()
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
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        assert_eq!(
            vec![
                Instruction {
                    kind: Kind::TurnOn,
                    c1: Coordinate { x: 0, y: 0 },
                    c2: Coordinate { x: 999, y: 999 }
                },
                Instruction {
                    kind: Kind::Toggle,
                    c1: Coordinate { x: 0, y: 0 },
                    c2: Coordinate { x: 999, y: 0 }
                },
                Instruction {
                    kind: Kind::TurnOff,
                    c1: Coordinate { x: 499, y: 499 },
                    c2: Coordinate { x: 500, y: 500 }
                },
            ],
            result
        );
    }

    #[test]
    fn part_1() {
        let expected = 998_996;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 1_001_996;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(&input);
        assert_eq!(result, expected)
    }
}
