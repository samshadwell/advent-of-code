use adv_code_2016::start_day;
use anyhow::{Result, anyhow, bail};
use const_format::concatcp;
use nom::character::complete::{line_ending, u32 as nom_u32};
use nom::multi::separated_list0;
use nom::{Finish, IResult};
use nom::{Parser, branch::alt, bytes::complete::tag, sequence::preceded};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::{collections::HashMap, time::Instant};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct BotId(u32);
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct OutputId(u32);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Destination {
    Bot(BotId),
    Output(OutputId),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Initial {
    value: u32,
    bot_id: BotId,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Sort {
    low: Destination,
    high: Destination,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Initial(Initial),
    Sort { bot_id: BotId, sort: Sort },
}

#[derive(Debug, PartialEq, Eq)]
struct Flow {
    initials: Vec<Initial>,
    bots: HashMap<BotId, Sort>,
}

fn destination_parser(s: &str) -> IResult<&str, Destination> {
    alt((
        preceded(tag("bot "), nom_u32).map(|l| Destination::Bot(BotId(l))),
        preceded(tag("output "), nom_u32).map(|l| Destination::Output(OutputId(l))),
    ))
    .parse(s)
}

fn parse(input: &str) -> Result<Flow> {
    let instruction_parser = alt((
        preceded(tag("value "), (nom_u32, tag(" goes to bot "), nom_u32)).map(
            |(value, _, bot_id)| {
                Instruction::Initial(Initial {
                    value,
                    bot_id: BotId(bot_id),
                })
            },
        ),
        (
            tag("bot "),
            nom_u32,
            tag(" gives low to "),
            destination_parser,
            tag(" and high to "),
            destination_parser,
        )
            .map(|(_, bot_id, _, low, _, high)| Instruction::Sort {
                bot_id: BotId(bot_id),
                sort: Sort { low, high },
            }),
    ));

    let (_, instructions) = separated_list0(line_ending, instruction_parser)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;

    let mut initials = Vec::new();
    let mut bots = HashMap::new();
    for i in instructions {
        match i {
            Instruction::Initial(i) => {
                initials.push(i);
            }
            Instruction::Sort { bot_id, sort } => {
                bots.insert(bot_id, sort);
            }
        }
    }

    Ok(Flow { initials, bots })
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum BotState {
    Empty,
    One(u32),
    Two(u32, u32),
}

struct State {
    bots: HashMap<BotId, BotState>,
    outputs: HashMap<OutputId, u32>,
}

impl State {
    fn new() -> Self {
        Self {
            bots: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    fn get_bot(&self, bot_id: BotId) -> BotState {
        *self.bots.get(&bot_id).unwrap_or(&BotState::Empty)
    }

    fn give(&mut self, bot: BotId, value: u32) -> Result<()> {
        let bs = self.bots.entry(bot).or_insert(BotState::Empty);
        *bs = match bs {
            BotState::Empty => BotState::One(value),
            BotState::One(a) => BotState::Two(*a, value),
            BotState::Two(_, _) => bail!("cannot give got more than 2 values"),
        };
        Ok(())
    }

    fn compute(&mut self, flow: &Flow) -> Result<()> {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        for Initial { bot_id, value } in &flow.initials {
            self.give(*bot_id, *value)?;
            queue.push_back(*bot_id);
        }

        while let Some(bot_id) = queue.pop_front() {
            if seen.contains(&bot_id) {
                continue;
            }
            match self.get_bot(bot_id) {
                BotState::Empty | BotState::One(_) => {}
                BotState::Two(a, b) => {
                    seen.insert(bot_id);
                    let Some(rule) = flow.bots.get(&bot_id) else {
                        bail!("no rule defined for bot with ID {bot_id:?}")
                    };
                    let (lesser, greater) = match a.cmp(&b) {
                        Ordering::Less => (a, b),
                        Ordering::Equal | Ordering::Greater => (b, a),
                    };

                    match rule.low {
                        Destination::Bot(dest) => {
                            self.give(dest, lesser)?;
                            queue.push_back(dest);
                        }
                        Destination::Output(dest) => {
                            self.outputs.insert(dest, lesser);
                        }
                    }
                    match rule.high {
                        Destination::Bot(dest) => {
                            self.give(dest, greater)?;
                            queue.push_back(dest);
                        }
                        Destination::Output(dest) => {
                            self.outputs.insert(dest, greater);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn part1(flow: &Flow, needle_a: u32, needle_b: u32) -> Result<BotId> {
    let mut state = State::new();
    state.compute(flow)?;
    for (bot_id, bs) in state.bots {
        match bs {
            BotState::Two(a, b)
                if (a == needle_a && b == needle_b) || (a == needle_b && b == needle_a) =>
            {
                return Ok(bot_id);
            }
            BotState::Empty | BotState::One(_) | BotState::Two(_, _) => {}
        }
    }
    Err(anyhow!("found no matching bots"))
}

fn part2(flow: &Flow) -> Result<u32> {
    let mut state = State::new();
    state.compute(flow)?;
    let Some(o_0) = state.outputs.get(&OutputId(0)) else {
        bail!("no output 0 found")
    };
    let Some(o_1) = state.outputs.get(&OutputId(1)) else {
        bail!("no output 1 found")
    };
    let Some(o_2) = state.outputs.get(&OutputId(2)) else {
        bail!("no output 2 found")
    };
    Ok(o_0 * o_1 * o_2)
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
    let result = part1(&input, 61, 17)?;
    println!("Result = {result:?}");
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
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Flow {
                initials: vec![
                    Initial {
                        value: 5,
                        bot_id: BotId(2)
                    },
                    Initial {
                        value: 3,
                        bot_id: BotId(1)
                    },
                    Initial {
                        value: 2,
                        bot_id: BotId(2)
                    },
                ],
                bots: HashMap::from([
                    (
                        BotId(2),
                        Sort {
                            low: Destination::Bot(BotId(1)),
                            high: Destination::Bot(BotId(0))
                        }
                    ),
                    (
                        BotId(1),
                        Sort {
                            low: Destination::Output(OutputId(1)),
                            high: Destination::Bot(BotId(0))
                        }
                    ),
                    (
                        BotId(0),
                        Sort {
                            low: Destination::Output(OutputId(2)),
                            high: Destination::Output(OutputId(0))
                        }
                    )
                ])
            }
        )
    }

    #[test]
    fn part_1() {
        let input = super::parse(TEST).expect("parse succeeds");
        assert_eq!(part1(&input, 5, 2).unwrap(), BotId(2));
    }

    #[test]
    fn part_2() {
        let input = super::parse(TEST).expect("parse succeeds");
        assert_eq!(part2(&input).expect("succeeds"), 30);
    }
}
