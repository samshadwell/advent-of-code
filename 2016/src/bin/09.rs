use adv_code_2016::start_day;
use anyhow::Result;
use const_format::concatcp;
use itertools::Itertools;
use std::time::Instant;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn decompressed_length(s: &str) -> usize {
    let mut len = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let num_chars_s: String = chars.peeking_take_while(|c| c.is_numeric()).collect();
                let Ok(num_chars) = num_chars_s.parse::<usize>() else {
                    // tag is invalid, add consumed character length and keep iterating
                    len += 1 + num_chars_s.len();
                    continue;
                };

                if matches!(chars.peek(), Some('x')) {
                    chars.next();
                } else {
                    len += 1 + num_chars_s.len();
                    continue;
                }

                let repeats_s: String = chars.peeking_take_while(|c| c.is_numeric()).collect();
                let Ok(repeats) = repeats_s.parse::<usize>() else {
                    len += 1 + num_chars_s.len() + 1 + repeats_s.len();
                    continue;
                };

                if matches!(chars.peek(), Some(')')) {
                    chars.next(); // Consume the end of the marker
                    let copied_chars = chars.by_ref().take(num_chars);
                    len += copied_chars.count() * repeats;
                } else {
                    len += 1 + num_chars_s.len() + 1 + repeats_s.len();
                }
            }
            _ => {
                len += 1;
            }
        }
    }
    len
}

fn part1(input: &[&str]) -> usize {
    input.iter().map(|s| decompressed_length(s)).sum()
}

fn recursive_decompressed_len(s: &mut dyn Iterator<Item = char>) -> usize {
    let mut len = 0;
    let mut chars = s.peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let num_chars_s: String = chars.peeking_take_while(|c| c.is_numeric()).collect();
                let Ok(num_chars) = num_chars_s.parse::<usize>() else {
                    // tag is invalid, add consumed character length and keep iterating
                    len += 1 + num_chars_s.len();
                    continue;
                };

                if matches!(chars.peek(), Some('x')) {
                    chars.next();
                } else {
                    len += 1 + num_chars_s.len();
                    continue;
                }

                let repeats_s: String = chars.peeking_take_while(|c| c.is_numeric()).collect();
                let Ok(repeats) = repeats_s.parse::<usize>() else {
                    len += 1 + num_chars_s.len() + 1 + repeats_s.len();
                    continue;
                };

                if matches!(chars.peek(), Some(')')) {
                    chars.next(); // Consume the end of the marker
                    // NOTE: The following 2 lines + signature is the difference between this and part 1
                    let mut copied_chars = chars.by_ref().take(num_chars);
                    len += recursive_decompressed_len(&mut copied_chars) * repeats;
                } else {
                    len += 1 + num_chars_s.len() + 1 + repeats_s.len();
                }
            }
            _ => {
                len += 1;
            }
        }
    }
    len
}

fn part2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|s| recursive_decompressed_len(&mut s.chars()))
        .sum()
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read_to_string(INPUT_FILE)?;
    let input: Vec<_> = file.lines().collect();
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

    #[test]
    fn part_1() {
        assert_eq!(decompressed_length("ADVENT"), 6);
        assert_eq!(decompressed_length("A(1x5)BC"), 7);
        assert_eq!(decompressed_length("(3x3)XYZ"), 9);
        assert_eq!(decompressed_length("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(decompressed_length("(6x1)(1x3)A"), 6);
        assert_eq!(decompressed_length("X(8x2)(3x3)ABCY"), 18);

        // Incomplete markers
        assert_eq!(decompressed_length("(foo)(1bx10)(1x10"), 17);

        // Incomplete data sequences
        assert_eq!(decompressed_length("(2x100)a"), 100);
    }

    #[test]
    fn part_2() {
        assert_eq!(recursive_decompressed_len(&mut "(3x3)XYZ".chars()), 9);
        assert_eq!(
            recursive_decompressed_len(&mut "X(8x2)(3x3)ABCY".chars()),
            20
        );
        assert_eq!(
            recursive_decompressed_len(&mut "(27x12)(20x12)(13x14)(7x10)(1x12)A".chars()),
            241920
        );
        assert_eq!(
            recursive_decompressed_len(
                &mut "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".chars()
            ),
            445
        );
    }
}
