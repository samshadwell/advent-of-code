use adv_code_2016::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use nom::bytes::complete::take_while1;
use nom::character::complete::{alpha1, char, newline, u32 as nom_u32};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::{Finish, IResult, Parser};
use std::cmp::Reverse;
use std::time::Instant;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn parse_room_id(s: &str) -> IResult<&str, RoomId<'_>> {
    (
        take_while1(|c: char| c.is_ascii_lowercase() || c == '-'),
        nom_u32,
        delimited(char('['), alpha1, char(']')),
    )
        .map(|(encrypted_name, id, checksum)| RoomId {
            encrypted_name,
            id,
            checksum,
        })
        .parse(s)
}

fn parse(input: &str) -> Result<Vec<RoomId<'_>>> {
    let (_, room_ids) = separated_list1(newline, parse_room_id)
        .parse(input)
        .finish()
        .map_err(|e: nom::error::Error<_>| anyhow!("parsing error {e}"))?;

    Ok(room_ids)
}

#[derive(Debug, PartialEq, Eq)]
struct RoomId<'a> {
    encrypted_name: &'a str,
    id: u32,
    checksum: &'a str,
}

impl RoomId<'_> {
    fn is_valid(&self) -> bool {
        let char_counts = self.encrypted_name.chars().filter(|c| *c != '-').counts();
        let correct_checksum = char_counts
            .into_iter()
            .sorted_by_key(|(c, n)| (Reverse(*n), *c))
            .take(5)
            .map(|(c, _)| c)
            .join("");
        correct_checksum == self.checksum
    }

    fn decrypt(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    let offset = u32::from(c as u8 - b'a');
                    let shifted = ((offset + self.id) % 26) as u8;
                    (b'a' + shifted) as char
                }
            })
            .join("")
    }
}

fn part1(input: &[RoomId]) -> u32 {
    input.iter().filter(|r| r.is_valid()).map(|r| r.id).sum()
}

fn part2(input: &[RoomId]) -> Option<u32> {
    input
        .iter()
        // Found the correct name by going through all the valid decrypted, names
        .find(|r| r.is_valid() && r.decrypt() == "northpole object storage ")
        .map(|r| r.id)
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
    let result = part1(&input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input).ok_or_else(|| anyhow!("did not find room"))?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
";

    #[test]
    fn parse() {
        let result = super::parse(TEST);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                RoomId {
                    encrypted_name: "aaaaa-bbb-z-y-x-",
                    id: 123,
                    checksum: "abxyz"
                },
                RoomId {
                    encrypted_name: "a-b-c-d-e-f-g-h-",
                    id: 987,
                    checksum: "abcde",
                },
                RoomId {
                    encrypted_name: "not-a-real-room-",
                    id: 404,
                    checksum: "oarel",
                },
                RoomId {
                    encrypted_name: "totally-real-room-",
                    id: 200,
                    checksum: "decoy",
                },
            ]
        )
    }

    #[test]
    fn part_1() {
        let expected = 1514;
        let input = super::parse(TEST).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result, expected)
    }

    #[test]
    fn decrypt() {
        let r = RoomId {
            encrypted_name: "qzmt-zixmtkozy-ivhz-",
            id: 343,
            checksum: "unused",
        };
        assert_eq!(r.decrypt(), "very encrypted name ")
    }
}
