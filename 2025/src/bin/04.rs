use adv_code_2025::grids::{Grid, Position};
use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse_grid<R: BufRead>(reader: R) -> Result<Grid<char>> {
    let vec: Result<Vec<Vec<char>>> = reader
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect();

    Ok(Grid::new(vec?))
}

fn count_adjacent_rolls(grid: &Grid<char>, position: &Position) -> usize {
    position
        .adjacent()
        .map(|p| grid.get(&p))
        .filter(|c| c.is_some_and(|c| *c == '@'))
        .count()
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let grid = parse_grid(reader)?;

    Ok(grid
        .all_positions()
        .map(|pos| (grid.get(&pos), pos))
        .map(|(val, pos)| match val {
            None => unreachable!("all positions should only return valid positions"),
            Some('@') if count_adjacent_rolls(&grid, &pos) < 4 => 1,
            _ => 0,
        })
        .sum())
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut grid = parse_grid(reader)?;

    let mut stack = Vec::new();
    grid.all_positions()
        .map(|pos| (grid.get(&pos), pos))
        .for_each(|(val, pos)| match val {
            None => unreachable!("all_positions should only return valid positions"),
            Some('@') if count_adjacent_rolls(&grid, &pos) < 4 => {
                stack.push(pos);
            }
            _ => {}
        });

    let mut num_removed = 0;
    while !stack.is_empty() {
        let to_process = stack.pop().unwrap();
        if let Some('@') = grid.get(&to_process)
            && count_adjacent_rolls(&grid, &to_process) < 4
        {
            grid.set(&to_process, '.')?;
            num_removed += 1;
            stack.extend(to_process.adjacent());
        }
    }

    Ok(num_removed)
}

fn main() -> Result<()> {
    let input = std::fs::read(INPUT_FILE)?;
    start_day(DAY);

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part1(input_file)?;
    let p1_elapsed = p1_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_elapsed);

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part2(input_file)?;
    let p2_elapsed = p2_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_elapsed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn part_1() {
        let expected = 13;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 43;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
