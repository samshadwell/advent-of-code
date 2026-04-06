use adv_code_2015::start_day;
use anyhow::Result;
use anyhow::anyhow;
use const_format::concatcp;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i32 as i32_nom, line_ending};
use nom::combinator::value;
use nom::multi::separated_list0;
use nom::sequence::preceded;
use nom::{Finish, Parser};
use std::time::Instant;

const DAY: &str = "23";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Register {
    A,
    B,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

fn register_parser(s: &str) -> IResult<&str, Register> {
    alt((value(Register::A, char('a')), value(Register::B, char('b')))).parse(s)
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let instruction_parser = alt((
        preceded(tag("hlf "), register_parser).map(Instruction::Hlf),
        preceded(tag("tpl "), register_parser).map(Instruction::Tpl),
        preceded(tag("inc "), register_parser).map(Instruction::Inc),
        preceded(tag("jmp "), i32_nom).map(Instruction::Jmp),
        (tag("jie "), register_parser, tag(", "), i32_nom)
            .map(|(_, r, _, i)| Instruction::Jie(r, i)),
        (tag("jio "), register_parser, tag(", "), i32_nom)
            .map(|(_, r, _, i)| Instruction::Jio(r, i)),
    ));

    let (_, instructions) = separated_list0(line_ending, instruction_parser)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;
    Ok(instructions)
}

#[derive(Default, Clone, Copy)]
struct MachineState {
    program_counter: i32,
    a: u32,
    b: u32,
}

impl std::ops::Index<Register> for MachineState {
    type Output = u32;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b,
        }
    }
}

impl std::ops::IndexMut<Register> for MachineState {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}

fn simulate(instructions: &[Instruction], state: &mut MachineState) {
    loop {
        if state.program_counter < 0 {
            return;
        }
        // At this point value must be non-negative
        #[allow(clippy::cast_sign_loss)]
        match instructions.get(state.program_counter as usize) {
            None => {
                return;
            }
            Some(Instruction::Hlf(r)) => {
                state[*r] /= 2;
            }
            Some(Instruction::Tpl(r)) => {
                state[*r] *= 3;
            }
            Some(Instruction::Inc(r)) => {
                state[*r] += 1;
            }
            Some(Instruction::Jmp(i)) => {
                state.program_counter += i;
                continue;
            }
            Some(Instruction::Jie(r, i)) => {
                if state[*r].is_multiple_of(2) {
                    state.program_counter += i;
                    continue;
                }
            }
            Some(Instruction::Jio(r, i)) => {
                if state[*r] == 1 {
                    state.program_counter += i;
                    continue;
                }
            }
        }
        state.program_counter += 1;
    }
}

fn part1(instructions: &[Instruction]) -> u32 {
    let mut state = MachineState::default();
    simulate(instructions, &mut state);
    state.b
}

fn part2(instructions: &[Instruction]) -> u32 {
    let mut state = MachineState {
        a: 1,
        ..MachineState::default()
    };
    simulate(instructions, &mut state);
    state.b
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read_to_string(INPUT_FILE)?;
    let instructions = parse(&file)?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&instructions);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&instructions);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
inc a
jio a, +2
tpl a
inc a
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                Instruction::Inc(Register::A),
                Instruction::Jio(Register::A, 2),
                Instruction::Tpl(Register::A),
                Instruction::Inc(Register::A)
            ]
        )
    }

    #[test]
    fn simulate_test() {
        let input = super::parse(TEST).expect("parse succeeds");
        let mut state = MachineState::default();
        simulate(&input, &mut state);
        assert_eq!(state.a, 2);
        assert_eq!(state.b, 0);
    }
}
