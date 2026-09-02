use adv_code_2016::start_day;
use anyhow::Result;
use anyhow::anyhow;
use const_format::concatcp;
use nom::Finish;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, u16 as u16_parser},
    multi::separated_list0,
};
use std::time::Instant;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Rect(u16, u16),
    RotRow(u16, u16),
    RotCol(u16, u16),
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let instruction_parser = alt((
        (tag("rect "), u16_parser, char('x'), u16_parser)
            .map(|(_, a, _, b)| Instruction::Rect(a, b)),
        (tag("rotate row y="), u16_parser, tag(" by "), u16_parser)
            .map(|(_, a, _, b)| Instruction::RotRow(a, b)),
        (tag("rotate column x="), u16_parser, tag(" by "), u16_parser)
            .map(|(_, a, _, b)| Instruction::RotCol(a, b)),
    ));
    let (_, instructions) = separated_list0(line_ending, instruction_parser)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;
    Ok(instructions)
}

fn print(display: &Vec<Vec<bool>>) {
    for row in display {
        for light in row {
            match light {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
    println!();
}

#[allow(clippy::indexing_slicing)]
fn part1(display: &mut [Vec<bool>], input: &[Instruction]) -> usize {
    for inst in input {
        match inst {
            Instruction::Rect(a, b) => {
                for row in display.iter_mut().take(*b as usize) {
                    for cell in row.iter_mut().take(*a as usize) {
                        *cell = true;
                    }
                }
            }
            Instruction::RotRow(r, k) => display[*r as usize].rotate_right(*k as usize),
            Instruction::RotCol(c, k) => {
                let c = *c as usize;
                let mut col: Vec<_> = display.iter().map(|r| r[c]).collect();
                col.rotate_right(*k as usize);
                for (row, new) in display.iter_mut().zip(col) {
                    row[c] = new;
                }
            }
        }
    }

    display.iter().flatten().filter(|b| **b).count()
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read_to_string(INPUT_FILE)?;
    let input = parse(&file)?;
    let mut display = vec![vec![false; 50]; 6];
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&mut display, &input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    print(&display);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                Instruction::Rect(3, 2),
                Instruction::RotCol(1, 1),
                Instruction::RotRow(0, 4),
                Instruction::RotCol(1, 1),
            ]
        );
    }

    #[test]
    fn part_1() {
        let input = super::parse(TEST).expect("parse succeeds");
        let mut display = vec![vec![false; 7]; 3];
        let expected = 6;
        let result = part1(&mut display, &input);
        assert_eq!(result, expected)
    }
}
