use adv_code_2016::start_day;
use anyhow::Result;
use anyhow::anyhow;
use const_format::concatcp;
use nom::character::complete::{char, i32 as nom_i32};
use nom::{Finish, Parser};
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::separated_list0};
use std::collections::HashSet;
use std::time::Instant;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Turn {
    Right,
    Left,
}

enum Direction {
    North,
    South,
    East,
    West,
}

// Positive y is northwards, positive x is eastwards
struct State {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Default for State {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            direction: Direction::North,
        }
    }
}

impl State {
    const fn turn(&mut self, t: Turn) {
        self.direction = match (&self.direction, t) {
            (Direction::North, Turn::Right) | (Direction::South, Turn::Left) => Direction::East,
            (Direction::North, Turn::Left) | (Direction::South, Turn::Right) => Direction::West,
            (Direction::East, Turn::Right) | (Direction::West, Turn::Left) => Direction::South,
            (Direction::East, Turn::Left) | (Direction::West, Turn::Right) => Direction::North,
        };
    }

    const fn do_move(&mut self, amount: i32) {
        match self.direction {
            Direction::North => self.y += amount,
            Direction::South => self.y -= amount,
            Direction::East => self.x += amount,
            Direction::West => self.x -= amount,
        }
    }
}

type Instruction = (Turn, i32);

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let (_, directions) = separated_list0(
        tag(", "),
        (
            alt((value(Turn::Right, char('R')), value(Turn::Left, char('L')))),
            nom_i32,
        ),
    )
    .parse(input)
    .finish()
    .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;
    Ok(directions)
}

fn part1(input: &[Instruction]) -> i32 {
    let mut state = State::default();
    for (turn, amount) in input {
        state.turn(*turn);
        state.do_move(*amount);
    }
    state.x.abs() + state.y.abs()
}

fn part2(input: &[Instruction]) -> Option<i32> {
    let mut state = State::default();
    let mut visited = HashSet::new();
    visited.insert((state.x, state.y));
    for (turn, amount) in input {
        state.turn(*turn);
        for _ in 0..*amount {
            state.do_move(1);
            let new = visited.insert((state.x, state.y));
            if !new {
                return Some(state.x.abs() + state.y.abs());
            }
        }
    }
    None
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
    let result = part2(&input).ok_or_else(|| anyhow!("did not visit any location twice"))?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
R5, L5, R5, R3
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                (Turn::Right, 5),
                (Turn::Left, 5),
                (Turn::Right, 5),
                (Turn::Right, 3)
            ]
        )
    }

    #[test]
    fn part_1() {
        let expected = 12;
        let input = super::parse(TEST).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = Some(4);
        let input = super::parse("R8, R4, R4, R8").expect("parse succeeds");
        let result = part2(&input);
        assert_eq!(result, expected)
    }
}
