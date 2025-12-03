use adv_code_2025::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn convert_ascii_digit(c: u8) -> Result<u128> {
    if !c.is_ascii_digit() {
        return Err(anyhow!("non-digit byte given: {}", c));
    }
    Ok(c as u128 - '0' as u128)
}

fn max_joltage(bank: &str, num_digits: usize) -> Result<u128> {
    if !bank.chars().all(|c| c.is_ascii_digit()) {
        return Err(anyhow!("expected digit-only string, got: {}", bank));
    }

    if num_digits == 0 {
        return Err(anyhow!("require num_digits to be at least 1, got 0"));
    }

    if bank.len() < num_digits {
        return Err(anyhow!(
            "invalid number digits in battery bank. Required at least {}, got {}",
            num_digits,
            bank.len()
        ));
    }

    let bytes = bank.as_bytes();
    let mut digits: Vec<Option<u8>> = vec![None; num_digits];

    'outer: for idx in 0..bytes.len() {
        let b = bytes[idx];
        'inner: for d_idx in 0..num_digits {
            // Can only "take" a digit if we have enough to fill in the rest of its digits
            let last_idx_for_digit = bytes.len() - (num_digits - d_idx);
            if idx > last_idx_for_digit {
                continue 'inner;
            }

            match digits[d_idx] {
                // We have no current value for this digit, anything is better
                None => {
                    digits[d_idx] = Some(b);
                    continue 'outer;
                }
                // Have a current value and this one is better, take it
                Some(curr) if curr < b => {
                    for d in digits.iter_mut().skip(d_idx) {
                        *d = None
                    }
                    digits[d_idx] = Some(b);
                    continue 'outer;
                }
                // Our current value is equal or better, consider for the next digit
                Some(_) => {
                    continue 'inner;
                }
            }
        }
    }

    if !digits.iter().all(|o| o.is_some()) {
        unreachable!("programmer error: all digits should have values")
    }

    let mut result = 0;
    for (i, d) in digits.iter().enumerate() {
        let contrib =
            10_u128.pow((num_digits - i - 1) as u32) * convert_ascii_digit(d.unwrap()).unwrap();
        result += contrib;
    }
    Ok(result)
}

fn solve<R: BufRead>(reader: R, num_digits: usize) -> Result<u128> {
    Ok(reader
        .lines()
        .flat_map(|l| max_joltage(&l?, num_digits))
        .sum())
}

fn part1<R: BufRead>(reader: R) -> Result<u128> {
    solve(reader, 2)
}

fn part2<R: BufRead>(reader: R) -> Result<u128> {
    solve(reader, 12)
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
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn part_1() {
        let expected = 357;
        let result = part1(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 3121910778619;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
