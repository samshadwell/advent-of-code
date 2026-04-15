use adv_code_2016::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use nom::character::complete::{multispace0, multispace1, newline, u32 as nom_u32};
use nom::multi::{many0, separated_list0};
use nom::{Finish, IResult, Parser};
use std::time::Instant;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, Clone, PartialEq, Eq)]
struct Triangle(u32, u32, u32);

impl Triangle {
    fn is_valid(&self) -> bool {
        let mut sides = [self.0, self.1, self.2];
        sides.sort_unstable();
        sides[2] < sides[0] + sides[1]
    }
}

fn parse_p1(input: &str) -> Result<Vec<Triangle>> {
    let triangle_parser = (
        multispace0,
        nom_u32,
        multispace1,
        nom_u32,
        multispace1,
        nom_u32,
    )
        .map(|(_, s1, _, s2, _, s3)| Triangle(s1, s2, s3));
    let (_, triangles) = separated_list0(newline, triangle_parser)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;

    Ok(triangles)
}

fn parse_line(s: &str) -> IResult<&str, (u32, u32, u32)> {
    (
        multispace0,
        nom_u32,
        multispace1,
        nom_u32,
        multispace1,
        nom_u32,
    )
        .map(|(_, s1, _, s2, _, s3)| (s1, s2, s3))
        .parse(s)
}

fn parse_p2(input: &str) -> Result<Vec<Triangle>> {
    let triple = (
        parse_line, newline, parse_line, newline, parse_line, newline,
    )
        .map(|((a1, b1, c1), _, (a2, b2, c2), _, (a3, b3, c3), _)| {
            vec![
                Triangle(a1, a2, a3),
                Triangle(b1, b2, b3),
                Triangle(c1, c2, c3),
            ]
        });
    let (_, triangles) = many0(triple)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;
    Ok(triangles.into_iter().flatten().collect())
}

fn num_valid(input: &[Triangle]) -> usize {
    input.iter().filter(|t| t.is_valid()).count()
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read_to_string(INPUT_FILE)?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let p1_input = parse_p1(&file)?;
    let result = num_valid(&p1_input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let p2_input = parse_p2(&file)?;
    let result = num_valid(&p2_input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
    5   10   25
  622  375   20
    7    7    7
    ";

    #[test]
    fn parse_p1() {
        let result = super::parse_p1(TEST);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                Triangle(5, 10, 25),
                Triangle(622, 375, 20),
                Triangle(7, 7, 7)
            ]
        )
    }

    #[test]
    fn part_1() {
        let input = super::parse_p1(TEST).expect("parse succeeds");
        assert_eq!(num_valid(&input), 1)
    }

    #[test]
    fn part_2() {
        let input = super::parse_p2(TEST).expect("parse succeeds");
        assert_eq!(
            input,
            vec![
                Triangle(5, 622, 7),
                Triangle(10, 375, 7),
                Triangle(25, 20, 7),
            ]
        );
        assert_eq!(num_valid(&input), 1)
    }
}
