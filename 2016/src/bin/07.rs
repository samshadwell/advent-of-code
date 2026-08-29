use adv_code_2016::start_day;
use anyhow::Result;
use const_format::concatcp;
use itertools::Itertools;
use std::time::Instant;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn part1(input: &[&str]) -> usize {
    input.iter().filter(|s| supports_tls(s)).count()
}

fn supports_tls(ip: &str) -> bool {
    let mut has_abba = false;
    let mut in_hypernet = false;
    for (a, b, c, d) in ip.chars().tuple_windows() {
        if a == '[' {
            in_hypernet = true;
            continue;
        }
        if a == ']' {
            in_hypernet = false;
            continue;
        }

        if a == d && b == c && a != b {
            has_abba = true;
            if in_hypernet {
                return false;
            }
        }
    }

    has_abba
}

fn part2(input: &[&str]) -> usize {
    input.iter().filter(|s| supports_ssl(s)).count()
}

fn supports_ssl(ip: &str) -> bool {
    let mut in_hypernet = false;
    for (a, b, c) in ip.chars().tuple_windows() {
        if a == ']' {
            in_hypernet = false;
            continue;
        }

        if in_hypernet {
            continue;
        }

        if a == '[' {
            in_hypernet = true;
            continue;
        }

        if a == c && a != b && hypernet_contains(ip, &format!("{b}{a}{b}")) {
            return true;
        }
    }
    false
}

fn hypernet_contains(ip: &str, needle: &str) -> bool {
    let mut in_hypernet = false;
    for (a, b, c) in ip.chars().tuple_windows() {
        if a == '[' {
            in_hypernet = true;
            continue;
        }
        if a == ']' {
            in_hypernet = false;
            continue;
        }
        if !in_hypernet {
            continue;
        }

        if format!("{a}{b}{c}") == needle {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read_to_string(INPUT_FILE)?;
    let input: Vec<&str> = file.lines().collect(); // TODO: Can I avoid collecting?
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
        assert_eq!(true, supports_tls("abba[mnop]qrst"));
        assert_eq!(false, supports_tls("abcd[bddb]xyyx"));
        assert_eq!(false, supports_tls("aaaa[qwer]tyui"));
        assert_eq!(true, supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn part_2() {
        assert_eq!(true, supports_ssl("aba[bab]xyz"));
        assert_eq!(false, supports_ssl("xyx[xyx]xyx"));
        assert_eq!(true, supports_ssl("aaa[kek]eke"));
        assert_eq!(true, supports_ssl("zazbz[bzb]cdb"));
    }
}
