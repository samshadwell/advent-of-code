use adv_code_2015::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse<R: BufRead>(reader: R) -> Result<Vec<String>> {
    reader
        .lines()
        .map(|l| l.with_context(|| "error reading line"))
        .collect()
}

fn unescaped_len(s: &str) -> Result<usize> {
    if !s.is_ascii() {
        return Err(anyhow!(
            "escaped_len only defined for ascii strings, found {s}"
        ));
    }

    let mut len = 0;
    let mut idx = 0;
    let bytes = s.as_bytes();
    while idx < bytes.len() {
        match bytes.get(idx) {
            Some(b'"') => {} // Unescaped quote, consume and move on
            Some(b'\\') => {
                let next = bytes.get(idx + 1);
                match next {
                    Some(b'\\' | b'"') => {
                        idx += 1;
                        len += 1;
                    }
                    Some(b'x') => {
                        // Escaped hex. Three extra chars to consume
                        idx += 3;
                        len += 1;
                    }
                    Some(c) => return Err(anyhow!("invalid escape sequence \\{c}")),
                    None => return Err(anyhow!("unterminated escape sequence")),
                }
            }
            Some(_) => len += 1,
            None => unreachable!(),
        }
        idx += 1;
    }

    Ok(len)
}

fn escaped_len(s: &str) -> usize {
    2 + s
        .chars()
        .map(|c| match c {
            '"' | '\\' => 2,
            _ => 1,
        })
        .sum::<usize>()
}

fn part1<T: AsRef<str>>(input: &[T]) -> Result<usize> {
    input
        .iter()
        .map(|s| Ok(s.as_ref().len() - unescaped_len(s.as_ref())?))
        .sum()
}

fn part2<T: AsRef<str>>(input: &[T]) -> usize {
    input
        .iter()
        .map(|s| escaped_len(s.as_ref()) - s.as_ref().len())
        .sum()
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
    let result = part1(&input)?;
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

    const TEST: &str = r#""abc"
"aaa\"aaa"
"#;

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2)
    }

    #[test]
    fn test_unescaped_len() {
        assert_eq!(0, unescaped_len(r#""""#).unwrap());
        assert_eq!(3, unescaped_len(r#""abc""#).unwrap());
        assert_eq!(7, unescaped_len(r#""aaa\"aaa""#).unwrap());
        assert_eq!(1, unescaped_len(r#""\x27""#).unwrap());
    }

    #[test]
    fn test_escaped_len() {
        assert_eq!(6, escaped_len(r#""""#));
        assert_eq!(9, escaped_len(r#""abc""#));
        assert_eq!(16, escaped_len(r#""aaa\"aaa""#));
        assert_eq!(11, escaped_len(r#""\x27""#));
    }
}
