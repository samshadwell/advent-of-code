use adv_code_2015::grids::Grid;
use adv_code_2015::grids::Position;
use adv_code_2015::start_day;
use anyhow::Result;
use anyhow::anyhow;
use const_format::concatcp;
use nom::Finish;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list0;
use std::time::Instant;

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse(input: &str) -> Result<Grid<bool>> {
    let (_, values) = separated_list0(
        line_ending,
        many1(alt((value(true, char('#')), value(false, char('.'))))),
    )
    .parse(input)
    .finish()
    .map_err(|e: nom::error::Error<_>| anyhow!("parsing error: {e}"))?;
    Grid::new(values)
}

fn next_state(pos: &Position, state: &Grid<bool>) -> Option<bool> {
    let on_neighbors = pos
        .adjacent()
        .map(|nbr| state.get(&nbr))
        .filter(|opt| opt.is_some_and(|&b| b))
        .count();

    match state.get(pos) {
        None => None,
        Some(true) => Some(on_neighbors == 2 || on_neighbors == 3),
        Some(false) => Some(on_neighbors == 3),
    }
}

fn next_step(initial: &Grid<bool>, buf: &mut Grid<bool>) -> Result<()> {
    for pos in initial.all_positions() {
        buf.set(
            &pos,
            next_state(&pos, initial).expect("pos references valid position in initial"),
        )?;
    }
    Ok(())
}

fn part1(initial: &Grid<bool>, num_steps: usize) -> usize {
    let mut curr = initial.clone();
    let mut next = initial.clone();
    for _ in 0..num_steps {
        next_step(&curr, &mut next).expect("next and curr are same size");
        (curr, next) = (next, curr);
    }
    curr.values().filter(|&&v| v).count()
}

fn part2(initial: &Grid<bool>, num_steps: usize) -> usize {
    let corners = vec![
        Position::new(0, 0),
        Position::new(0, initial.num_cols() - 1),
        Position::new(initial.num_rows() - 1, 0),
        Position::new(initial.num_rows() - 1, initial.num_cols() - 1),
    ];
    let mut curr = initial.clone();
    for corner in &corners {
        curr.set(corner, true).expect("corner is valid coordinate");
    }
    let mut next = initial.clone();
    for _ in 0..num_steps {
        next_step(&curr, &mut next).expect("next and curr are same size");
        (curr, next) = (next, curr);
        for corner in &corners {
            curr.set(corner, true).expect("corner is valid coordinate");
        }
    }
    curr.values().filter(|&&v| v).count()
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
    let result = part1(&input, 100);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input, 100);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
.#.#.#
...##.
#....#
..#...
#.#..#
####..
";

    #[test]
    fn parse() {
        let result = super::parse(&TEST);
        assert!(result.is_ok());
        let grid = result.unwrap();
        assert_eq!(
            grid.all_positions()
                .map(|p| grid.get(&p))
                .map(Option::unwrap)
                .map(|&b| b)
                .collect::<Vec<_>>(),
            vec![
                false, true, false, true, false, true, false, false, false, true, true, false,
                true, false, false, false, false, true, false, false, true, false, false, false,
                true, false, true, false, false, true, true, true, true, true, false, false
            ]
        )
    }

    #[test]
    fn part_1() {
        let expected = 4;
        let input = super::parse(&TEST).expect("parse succeeds");
        let result = part1(&input, 4);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 17;
        let input = super::parse(&TEST).expect("parse succeeds");
        let result = part2(&input, 5);
        assert_eq!(result, expected)
    }
}
