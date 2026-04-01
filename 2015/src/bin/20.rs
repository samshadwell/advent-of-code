use adv_code_2015::start_day;
use std::iter::once;
use std::time::Instant;

const DAY: &str = "19";

fn factors(n: u32) -> impl Iterator<Item = u32> {
    (1..=n.isqrt())
        .filter(move |&i| n.is_multiple_of(i))
        .flat_map(move |i| {
            if i == n / i {
                // i is square root, only add to list of factors once
                Box::new(once(i)) as Box<dyn Iterator<Item = u32>>
            } else {
                Box::new(once(i).chain(once(n / i)))
            }
        })
}

fn part1(threshold: u32) -> u32 {
    for house_num in 1.. {
        let num_presents = factors(house_num).sum::<u32>() * 10;
        if num_presents >= threshold {
            return house_num;
        }
    }
    unreachable!("for loop will go until it finds a suitable house")
}

fn part2(threshold: u32) -> u32 {
    for house_num in 1.. {
        let num_presents = factors(house_num)
            .filter(|elf| house_num / elf <= 50)
            .sum::<u32>()
            * 11;
        if num_presents >= threshold {
            return house_num;
        }
    }
    unreachable!("for loop will go until it finds a suitable house")
}

fn main() {
    start_day(DAY);

    let input = 36_000_000;

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(part1(70), 4);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(70), 4);
    }
}
