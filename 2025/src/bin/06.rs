use adv_code_2025::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

enum Operation {
    Addition,
    Multiplication,
}

impl Operation {
    fn identity(&self) -> u64 {
        match self {
            Self::Addition => 0,
            Self::Multiplication => 1,
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Addition => a + b,
            Self::Multiplication => a * b,
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Addition),
            "*" => Ok(Operation::Multiplication),
            _ => Err(anyhow!("unrecognized operator {}", s)),
        }
    }
}

// Given reader of input file, transpose it so that inner vectors contain
// entire equations, the outer vector is a collection of all equations
fn group_equations<R: BufRead>(reader: R) -> Result<Vec<Vec<String>>> {
    let mut transposed: Vec<Vec<String>> = Vec::new();
    for res in reader.lines() {
        let line = res?;
        line.split_whitespace().enumerate().for_each(|(idx, s)| {
            if transposed.len() <= idx {
                transposed.push(Vec::new());
            }
            transposed
                .get_mut(idx)
                .expect("inner vector exists via above")
                .push(s.to_string());
        })
    }
    Ok(transposed)
}

fn part1_calc(equations: Vec<Vec<String>>) -> Result<u64> {
    equations
        .iter()
        .map(|equation| {
            let mut rev = equation.iter().rev();
            let op: Operation = rev
                .next()
                .ok_or_else(|| anyhow!("equation has no elements"))?
                .parse()?;
            rev.try_fold(op.identity(), |acc, x| {
                x.parse()
                    .map(|val| op.apply(acc, val))
                    .map_err(anyhow::Error::from)
            })
        })
        .sum()
}

fn part1<R: BufRead>(reader: R) -> Result<u64> {
    let equations = group_equations(reader)?;
    part1_calc(equations)
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        input.push(line?.chars().collect());
    }
    if input.is_empty() {
        return Ok(0);
    }

    let max_cols = input
        .iter()
        .map(|v| v.len())
        .max()
        .expect("at least one row in input");

    let mut curr_nums: Vec<u64> = Vec::new();
    let mut curr = 0;
    let mut total = 0;
    // Iterate right-to-left since operations are in bottom-left corners
    'outer: for col in (0..max_cols).rev() {
        for row in &input {
            if let Some(c) = row.get(col) {
                match c {
                    '0'..='9' => {
                        let v = c.to_digit(10).expect("known digit");
                        curr = (curr * 10) + v as u64;
                    }
                    '+' | '*' => {
                        curr_nums.push(curr);
                        total += if *c == '+' {
                            curr_nums.iter().sum::<u64>()
                        } else {
                            curr_nums.iter().product()
                        };
                        curr_nums.clear();
                        curr = 0;
                        continue 'outer;
                    }
                    _ => {}
                }
            }
        }
        // End of column
        if curr > 0 {
            curr_nums.push(curr);
        }
        curr = 0;
    }
    Ok(total)
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
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn part_1() {
        let expected = 4277556;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 3263827;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
