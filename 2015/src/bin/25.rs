use adv_code_2015::start_day;
use std::time::Instant;

const DAY: &str = "25";

#[derive(Debug, Clone, Copy)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn ordinal(&self) -> usize {
        let diag = self.row + self.col - 1;
        (1..diag).sum::<usize>() + self.col
    }
}

fn part1(first: usize, loc: &Location) -> usize {
    let mut val = first;
    for _ in 2..=loc.ordinal() {
        val = (val * 252_533) % 33_554_393;
    }
    val
}

fn main() {
    start_day(DAY);

    let initial = 20_151_125;
    let input = Location {
        row: 2981,
        col: 3075,
    };

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(initial, &input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ordinal() {
        assert_eq!(Location { row: 1, col: 1 }.ordinal(), 1);
        assert_eq!(Location { row: 1, col: 5 }.ordinal(), 15);
        assert_eq!(Location { row: 3, col: 4 }.ordinal(), 19);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(20151125, &Location { row: 1, col: 1 }), 20151125);
        assert_eq!(part1(20151125, &Location { row: 3, col: 2 }), 8057251);
        assert_eq!(part1(20151125, &Location { row: 6, col: 4 }), 24659492);
    }
}
