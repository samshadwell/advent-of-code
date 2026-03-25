use adv_code_2015::start_day;
use anyhow::Result;
use anyhow::anyhow;
use const_format::concatcp;
use itertools::Itertools;
use nom::Finish;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32};
use nom::combinator::all_consuming;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

type Input = Vec<Ingredient>;

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

fn parse_line(s: &str) -> IResult<&str, Ingredient> {
    all_consuming((
        alpha1,
        tag(": capacity "),
        i32,
        tag(", durability "),
        i32,
        tag(", flavor "),
        i32,
        tag(", texture "),
        i32,
        tag(", calories "),
        i32,
    ))
    .parse(s)
    .map(
        |(res, (name, _, capacity, _, durability, _, flavor, _, texture, _, calories))| {
            (
                res,
                Ingredient {
                    name: name.to_owned(),
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            )
        },
    )
}

fn solve<F>(input: &Input, score: F) -> i32
where
    F: Fn(&[(&Ingredient, i32)]) -> i32,
{
    // Degenerate case: only 1 ingredient
    if input.len() == 1 {
        return score(&[(input.first().unwrap(), 100)]);
    }

    let mut amounts: Vec<i32> = Vec::with_capacity(input.len());
    (0..=100)
        .combinations_with_replacement(input.len() - 1)
        .map(|mut v| {
            amounts.clear();
            v.sort_unstable();
            // Each v_i represents at which "teaspoon" ingredient ends, exclusive.
            // v must be nonempty by construction
            #[allow(clippy::indexing_slicing)]
            amounts.push(v[0]);
            amounts.extend(v.iter().tuple_windows().map(|(&s, &e)| e - s));
            amounts.push(100 - amounts.iter().sum::<i32>());

            score(
                &input
                    .iter()
                    .zip(amounts.iter().copied())
                    .collect::<Vec<_>>(),
            )
        })
        .max()
        .unwrap_or(0)
}

fn score_and_calories(recipe: &[(&Ingredient, i32)]) -> (i32, i32) {
    let (cap, dur, fla, tex, cal) = recipe.iter().fold(
        (0, 0, 0, 0, 0),
        |(cap, dur, fla, tex, cal), &(i, amount)| {
            (
                cap + i.capacity * amount,
                dur + i.durability * amount,
                fla + i.flavor * amount,
                tex + i.texture * amount,
                cal + i.calories * amount,
            )
        },
    );

    (cap.max(0) * dur.max(0) * fla.max(0) * tex.max(0), cal)
}

fn p1_score(recipe: &[(&Ingredient, i32)]) -> i32 {
    score_and_calories(recipe).0
}

fn p2_score(recipe: &[(&Ingredient, i32)]) -> i32 {
    let (score, cal) = score_and_calories(recipe);
    if cal == 500 { score } else { 0 }
}

fn part1(input: &Input) -> i32 {
    solve(input, p1_score)
}

fn part2(input: &Input) -> i32 {
    solve(input, p2_score)
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
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());

        let v = result.expect("parse succeeded");
        assert_eq!(2, v.len());
        assert_eq!(
            Some(&Ingredient {
                name: "Butterscotch".to_owned(),
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            }),
            v.get(0)
        );
        assert_eq!(
            Some(&Ingredient {
                name: "Cinnamon".to_owned(),
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            }),
            v.get(1)
        );
    }

    #[test]
    fn part_1() {
        let expected = 62842880;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 57600000;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(&input);
        assert_eq!(result, expected)
    }
}
