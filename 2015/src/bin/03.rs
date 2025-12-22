use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::collections::HashSet;
use std::time::Instant;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn do_move(curr: (i32, i32), dir: char) -> Result<(i32, i32)> {
    match dir {
        '<' => Ok((curr.0 - 1, curr.1)),
        '>' => Ok((curr.0 + 1, curr.1)),
        '^' => Ok((curr.0, curr.1 - 1)),
        'v' => Ok((curr.0, curr.1 + 1)),
        _ => Err(anyhow!("unknown direction: {dir}")),
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut seen = HashSet::from([(0, 0)]);
    input.chars().try_fold((0, 0), |pos, c| {
        let new = do_move(pos, c)?;
        seen.insert(new);
        anyhow::Ok(new)
    })?;
    Ok(seen.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut seen = HashSet::from([(0, 0)]);
    input
        .chars()
        .enumerate()
        .try_fold([(0, 0); 2], |mut positions, (idx, c)| {
            let which = idx % 2;
            let pos = positions
                .get_mut(which)
                .ok_or_else(|| anyhow!("index out of bounds"))?;
            *pos = do_move(*pos, c)?;
            seen.insert(*pos);
            anyhow::Ok(positions)
        })?;

    Ok(seen.len())
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let input = std::fs::read_to_string(INPUT_FILE)?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input)?;
    println!("Result = {result}");
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

    #[test]
    fn part_1() {
        assert_eq!(2, part1(">").unwrap());
        assert_eq!(4, part1("^>v<").unwrap());
        assert_eq!(2, part1("^v^v^v^v^v").unwrap());
    }

    #[test]
    fn part_2() {
        assert_eq!(3, part2("^v").unwrap());
        assert_eq!(3, part2("^>v<").unwrap());
        assert_eq!(11, part2("^v^v^v^v^v").unwrap());
    }
}
