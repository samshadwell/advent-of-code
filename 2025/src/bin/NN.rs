use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "NN"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

// TODO: Change result type and implement
fn part1<R: BufRead>(_reader: R) -> Result<i32> {
    Ok(0)
}

// TODO: Change result type and implement
fn part2<R: BufRead>(_reader: R) -> Result<i32> {
    Ok(0)
}

fn main() -> Result<()> {
    let input = std::fs::read(INPUT_FILE)?;
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part1(input_file)?;
    let p1_elapsed = p1_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_elapsed);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part2(input_file)?;
    let p2_elapsed = p2_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_elapsed);
    //endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input

    #[test]
    fn part_1() {
        // TODO: Modify expected
        let expected = 0;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        // TODO: Modify expected
        let expected = 0;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
