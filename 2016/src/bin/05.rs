use adv_code_2016::start_day;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{fmt::Write, time::Instant};

const DAY: &str = "05";

fn part1(input: &str, num_chars: usize) -> String {
    let mut results = Vec::with_capacity(num_chars);
    // chunk_size found by some rough hand-tuning. This seems to be the right order
    // of magnitude: approximately one "good" hash per 2 chunks
    let chunk_size = 512_000;
    let mut start = 0;

    while results.len() < num_chars {
        let mut chunk_matches = (start..start + chunk_size)
            .into_par_iter()
            .filter_map(|i| {
                let str = format!("{input}{i}");
                let digest = md5::compute(str);
                #[allow(clippy::indexing_slicing)]
                if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0 {
                    let sixth_nibble = digest[2] & 0x0F;
                    let sixth_char = char::from_digit(u32::from(sixth_nibble), 16).unwrap();
                    Some(sixth_char)
                } else {
                    None
                }
            })
            .collect();
        results.append(&mut chunk_matches);
        start += chunk_size;
    }

    results.into_iter().take(num_chars).collect()
}

fn format_password(pw: &[Option<char>]) -> String {
    let mut s = String::with_capacity(pw.len());
    for pos in pw {
        match pos {
            None => write!(&mut s, "_").expect("write to string succeeds"),
            Some(c) => write!(&mut s, "{c}").expect("write to string succeeds"),
        }
    }
    s
}

fn part2(input: &str, num_chars: u8) -> String {
    let mut result: Vec<Option<char>> = Vec::with_capacity(usize::from(num_chars));
    for _ in 0..num_chars {
        result.push(None);
    }
    let chunk_size = 512_000;
    let mut start = 0;

    loop {
        let chunk_matches: Vec<_> = (start..start + chunk_size)
            .into_par_iter()
            .filter_map(|i| {
                let str = format!("{input}{i}");
                let digest = md5::compute(str);
                #[allow(clippy::indexing_slicing)]
                if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0 {
                    let position = digest[2] & 0x0F;
                    // Seventh character is the high nibble of the 4th byte
                    let char_nibble = digest[3] >> 4;
                    let c = char::from_digit(u32::from(char_nibble), 16).expect("valid hex");
                    Some((usize::from(position), c))
                } else {
                    None
                }
            })
            .collect();

        for (i, c) in chunk_matches {
            if let Some(elem) = result.get_mut(i) {
                if elem.is_some() {
                    continue;
                }
                *elem = Some(c);
                println!("{}", format_password(&result));
                if result.iter().all(std::option::Option::is_some) {
                    return result.into_iter().flatten().collect();
                }
            }
        }
        start += chunk_size;
    }
}

fn main() {
    start_day(DAY);
    let input = "cxdnnyjw";

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(input, 8);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(input, 8);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "runs slowly"]
    fn part_1() {
        assert_eq!(part1("abc", 3), "18f")
    }

    #[test]
    #[ignore = "runs slowly"]
    fn part_2() {
        assert_eq!(part2("abc", 3), "05a")
    }
}
