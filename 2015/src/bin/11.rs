// I do lots of direct indexing in this file. It's tested and works, moving to .get would
// make it harder to read
#![allow(clippy::indexing_slicing)]
use adv_code_2015::start_day;
use anyhow::{Result, bail};
use itertools::Itertools;
use nom::AsChar;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

const DAY: &str = "11";

#[derive(Clone)]
struct Password([u8; 8]);

const fn char_allowed(c: u8) -> bool {
    match c + b'a' {
        b'i' | b'o' | b'l' => false,
        b'a'..=b'z' => true,
        _ => false,
    }
}

impl Password {
    fn incr(&mut self) {
        self.0[7] += 1;
        for i in (0..=7).rev() {
            if self.0[i] == 26 {
                self.0[i] = 0;
                if i > 0 {
                    self.0[i - 1] += 1;
                }
            }
        }
    }

    fn is_valid(&self) -> bool {
        let all_allowed = self.0.iter().all(|&c| char_allowed(c));
        if !all_allowed {
            return false;
        }

        let has_straight = self
            .0
            .iter()
            .tuple_windows()
            .any(|(&a, &b, &c)| a + 1 == b && b + 1 == c);

        if !has_straight {
            return false;
        }

        for c1_start in 0..=6 {
            for c2_start in (c1_start + 2)..=6 {
                if self.0[c1_start] == self.0[c1_start + 1]
                    && self.0[c2_start] == self.0[c2_start + 1]
                {
                    return true;
                }
            }
        }

        false
    }

    fn next_valid(&self) -> Self {
        let mut next = self.clone();
        next.incr();
        while !next.is_valid() {
            next.incr();
        }
        next
    }
}

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if !s.is_ascii() {
            bail!("passwords must be ascii-only strings, got {s}");
        }
        if s.len() != 8 {
            bail!(
                "passwords must contain exactly 8 characters, got length {} for {s}",
                s.len(),
            )
        }

        let mut pw = [0u8; 8];
        for (i, c) in s.bytes().enumerate() {
            if !c.is_ascii_lowercase() {
                bail!("passwords must contain only lowercase letters, got character {c} in {s}")
            }
            pw[i] = c - b'a';
        }

        Ok(Self(pw))
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(0);
        for c in self.0 {
            s.push((c + b'a').as_char());
        }
        write!(f, "{s}")
    }
}

#[allow(clippy::missing_const_for_fn)]
fn part1(input: &Password) -> String {
    input.next_valid().to_string()
}

#[allow(clippy::missing_const_for_fn)]
fn part2(input: &Password) -> String {
    input.next_valid().next_valid().to_string()
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let input = Password::from_str("hxbxwxba")?;
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
    fn password_incr() {
        let mut aaa = Password::from_str("aaaaaaaa").expect("parse succeeds");
        aaa.incr();
        assert_eq!(aaa.to_string(), "aaaaaaab");

        // Single-character wrap
        let mut pw = Password::from_str("aaaaaaaz").expect("parse succeeds");
        pw.incr();
        assert_eq!(pw.to_string(), "aaaaaaba");

        // Full wrap zzz -> aaa
        let mut zzz = Password::from_str("zzzzzzzz").expect("parse succeeds");
        zzz.incr();
        assert_eq!(zzz.to_string(), "aaaaaaaa");
    }

    #[test]
    fn is_valid() {
        assert_eq!(false, Password::from_str("hijklmmn").unwrap().is_valid());
        assert_eq!(false, Password::from_str("abbceffg").unwrap().is_valid());
        assert_eq!(false, Password::from_str("abbcegjk").unwrap().is_valid());

        assert_eq!(true, Password::from_str("abcdffaa").unwrap().is_valid());
        assert_eq!(true, Password::from_str("ghjaabcc").unwrap().is_valid());
    }

    #[test]
    fn next_valid() {
        assert_eq!("abcdffaa", part1(&Password::from_str("abcdefgh").unwrap()));
        assert_eq!("ghjaabcc", part1(&Password::from_str("ghijklmn").unwrap()));
    }
}
