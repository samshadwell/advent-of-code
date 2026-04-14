use adv_code_2016::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use nom::{
    Finish, Parser,
    branch::alt,
    character::complete::{char, newline},
    combinator::value,
    multi::{many1, separated_list0},
};
use std::{fmt::Display, time::Instant};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

trait DoesMoves {
    fn do_move(self, m: Move) -> Self;
}

#[derive(Copy, Clone)]
struct NumericButton(u16);

impl NumericButton {
    fn new(val: u16) -> Result<Self> {
        match val {
            1..=9 => Ok(Self(val)),
            _ => Err(anyhow!("invalid button value")),
        }
    }
}

impl DoesMoves for NumericButton {
    fn do_move(self, m: Move) -> Self {
        match m {
            Move::Up => match self.0 {
                1..=3 => self,
                _ => Self::new(self.0 - 3).expect("in bounds by construction"),
            },
            Move::Down => match self.0 {
                7..=9 => self,
                _ => Self::new(self.0 + 3).expect("in bounds by construction"),
            },
            Move::Left => match self.0 {
                1 | 4 | 7 => self,
                _ => Self::new(self.0 - 1).expect("in bounds by construction"),
            },
            Move::Right => match self.0 {
                3 | 6 | 9 => self,
                _ => Self::new(self.0 + 1).expect("in bounds by construction"),
            },
        }
    }
}

impl Display for NumericButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone)]
struct FancyButton(char);

impl FancyButton {
    fn new(c: char) -> Result<Self> {
        match c {
            '0'..='9' | 'A'..='D' => Ok(Self(c)),
            _ => Err(anyhow!("invalid FancyButton character {c}")),
        }
    }
}

impl DoesMoves for FancyButton {
    fn do_move(self, m: Move) -> Self {
        match m {
            Move::Up => match self.0 {
                '1' | '2' | '4' | '5' | '9' => self,
                '3' => Self::new('1').expect("valid"),
                '6' => Self::new('2').expect("valid"),
                '7' => Self::new('3').expect("valid"),
                '8' => Self::new('4').expect("valid"),
                'A' => Self::new('6').expect("valid"),
                'B' => Self::new('7').expect("valid"),
                'C' => Self::new('8').expect("valid"),
                'D' => Self::new('B').expect("valid"),
                _ => unreachable!(),
            },
            Move::Down => match self.0 {
                '5' | 'A' | 'D' | 'C' | '9' => self,
                '1' => Self::new('3').expect("valid"),
                '2' => Self::new('6').expect("valid"),
                '3' => Self::new('7').expect("valid"),
                '4' => Self::new('8').expect("valid"),
                '6' => Self::new('A').expect("valid"),
                '7' => Self::new('B').expect("valid"),
                '8' => Self::new('C').expect("valid"),
                'B' => Self::new('D').expect("valid"),
                _ => unreachable!(),
            },
            Move::Left => match self.0 {
                '1' | '2' | '5' | 'A' | 'D' => self,
                '3' => Self::new('2').expect("valid"),
                '4' => Self::new('3').expect("valid"),
                '6' => Self::new('5').expect("valid"),
                '7' => Self::new('6').expect("valid"),
                '8' => Self::new('7').expect("valid"),
                '9' => Self::new('8').expect("valid"),
                'B' => Self::new('A').expect("valid"),
                'C' => Self::new('B').expect("valid"),
                _ => unreachable!(),
            },
            Move::Right => match self.0 {
                '1' | '4' | '9' | 'C' | 'D' => self,
                '2' => Self::new('3').expect("valid"),
                '3' => Self::new('4').expect("valid"),
                '5' => Self::new('6').expect("valid"),
                '6' => Self::new('7').expect("valid"),
                '7' => Self::new('8').expect("valid"),
                '8' => Self::new('9').expect("valid"),
                'A' => Self::new('B').expect("valid"),
                'B' => Self::new('C').expect("valid"),
                _ => unreachable!(),
            },
        }
    }
}

impl Display for FancyButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn parse(input: &str) -> Result<Vec<Vec<Move>>> {
    let (_, moves) = separated_list0(
        newline,
        many1(alt((
            value(Move::Up, char('U')),
            value(Move::Down, char('D')),
            value(Move::Left, char('L')),
            value(Move::Right, char('R')),
        ))),
    )
    .parse(input)
    .finish()
    .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;
    Ok(moves)
}

fn solve<B, T>(start: &B, input: &[T]) -> String
where
    B: DoesMoves + Copy + Display,
    T: AsRef<[Move]>,
{
    let mut seq = Vec::with_capacity(input.len());
    let mut button = *start;
    for line in input {
        for m in line.as_ref() {
            button = button.do_move(*m);
        }
        seq.push(button);
    }
    seq.into_iter().map(|b| b.to_string()).collect()
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
    let start_p1 = NumericButton::new(5)?;
    let result = solve(&start_p1, &input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let start_p2 = FancyButton::new('5')?;
    let result = solve(&start_p2, &input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
ULL
RRDDD
LURDL
UUUUD
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                vec![Move::Up, Move::Left, Move::Left],
                vec![Move::Right, Move::Right, Move::Down, Move::Down, Move::Down],
                vec![Move::Left, Move::Up, Move::Right, Move::Down, Move::Left],
                vec![Move::Up, Move::Up, Move::Up, Move::Up, Move::Down]
            ]
        )
    }

    #[test]
    fn part_1() {
        let start = NumericButton::new(5).expect("5 is valid button");
        let input = super::parse(TEST).expect("parse succeeds");
        let result = solve(&start, &input);
        assert_eq!(result, "1985".to_string())
    }

    #[test]
    fn part_2() {
        let start = FancyButton::new('5').expect("5 is valid button");
        let input = super::parse(TEST).expect("parse succeeds");
        let result = solve(&start, &input);
        assert_eq!(result, "5DB3".to_string())
    }
}
