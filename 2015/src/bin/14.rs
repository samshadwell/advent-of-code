use adv_code_2015::start_day;
use anyhow::Result;
use anyhow::anyhow;
use const_format::concatcp;
use nom::Finish;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u32};
use nom::combinator::all_consuming;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Reindeer {
    name: String,
    fly_speed: u32,
    fly_time: u32,
    rest_time: u32,
}

type Input = Vec<Reindeer>;

fn parse<R: BufRead>(reader: R) -> Result<Input> {
    reader
        .lines()
        .map(|l| {
            let line = l.map_err(|e| anyhow!("error reading line {e}"))?;
            let res = parse_line(&line)
                .finish()
                .map_err(|e| anyhow!("parse error {e}"))?;
            Ok(res.1)
        })
        .collect()
}

fn parse_line(s: &str) -> IResult<&str, Reindeer> {
    all_consuming((
        alpha1,
        tag(" can fly "),
        u32,
        tag(" km/s for "),
        u32,
        tag(" seconds, but then must rest for "),
        u32,
        tag(" seconds."),
    ))
    .parse(s)
    .map(
        |(res, (name, _, fly_speed, _, fly_time, _, rest_time, _))| {
            (
                res,
                Reindeer {
                    name: name.to_owned(),
                    fly_speed,
                    fly_time,
                    rest_time,
                },
            )
        },
    )
}

fn distance_traveled(reindeer: &Reindeer, num_seconds: u32) -> u32 {
    let mut flying = true;
    let mut seconds_left = num_seconds;
    let mut distance_traveled = 0;
    while seconds_left > 0 {
        if flying {
            let this_flight = reindeer.fly_time.min(seconds_left);
            distance_traveled += this_flight * reindeer.fly_speed;
            seconds_left -= this_flight;
        } else {
            let rest_time = reindeer.rest_time.min(seconds_left);
            seconds_left -= rest_time;
        }
        flying = !flying;
    }
    distance_traveled
}

fn part1(reindeer: &Input, num_seconds: u32) -> u32 {
    reindeer
        .iter()
        .map(|r| distance_traveled(r, num_seconds))
        .max()
        .unwrap_or(0)
}

fn part2(reindeer: &Input, num_second: u32) -> u32 {
    let mut num_points: HashMap<&Reindeer, u32> = HashMap::with_capacity(reindeer.len());
    let mut leaders: Vec<&Reindeer> = Vec::with_capacity(reindeer.len());
    for s in 1..=num_second {
        let mut leader_dist = 0;
        for r in reindeer {
            let dist = distance_traveled(r, s);
            if dist > leader_dist {
                leader_dist = dist;
                leaders.clear();
                leaders.push(r);
            } else if dist == leader_dist {
                leaders.push(r);
            }
        }

        for r in &leaders {
            num_points.entry(r).and_modify(|n| *n += 1).or_insert(1);
        }
        leaders.clear();
    }

    num_points.values().max().unwrap_or(&0).to_owned()
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::File::open(INPUT_FILE)?;
    let input = parse(BufReader::new(file))?;
    let num_seconds = 2503;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input, num_seconds);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input, num_seconds);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        let v = result.unwrap();
        assert_eq!(2, v.len());
        assert_eq!(
            Some(&Reindeer {
                name: "Comet".to_owned(),
                fly_speed: 14,
                fly_time: 10,
                rest_time: 127
            }),
            v.get(0)
        );
        assert_eq!(
            Some(&Reindeer {
                name: "Dancer".to_owned(),
                fly_speed: 16,
                fly_time: 11,
                rest_time: 162,
            }),
            v.get(1)
        )
    }

    #[test]
    fn part_1() {
        let expected = 1120;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input, 1000);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 689;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(&input, 1000);
        assert_eq!(result, expected)
    }
}
