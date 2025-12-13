use adv_code_2025::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Instant;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug)]
struct Coordinate {
    x: u64,
    y: u64,
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^(\d+),(\d+)$").expect("valid regex"));

        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid point format: {}", s))?;
        let x = caps[1].parse().context("parsing x")?;
        let y = caps[2].parse().context("parsing y")?;

        Ok(Self { x, y })
    }
}

fn parse_coordinates<R: BufRead>(reader: R) -> Result<Vec<Coordinate>> {
    reader.lines().map(|l| Coordinate::from_str(&l?)).collect()
}

fn enclosed_area(a: &Coordinate, b: &Coordinate) -> u64 {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn part1(coordinates: &[Coordinate]) -> Result<u64> {
    coordinates
        .iter()
        .tuple_combinations()
        .map(|(a, b)| enclosed_area(a, b))
        .max()
        .ok_or_else(|| anyhow!("no max exists, not enough elements given"))
}

#[derive(Default)]
struct CompressedGrid {
    x_remap: HashMap<u64, usize>,
    y_remap: HashMap<u64, usize>,
    grid: Vec<Vec<char>>,
}

impl fmt::Debug for CompressedGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // A helper wrapper to handle the specific formatting of the visual grid
        struct GridView<'a>(&'a Vec<Vec<char>>);

        impl<'a> fmt::Debug for GridView<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // Start with a newline so the grid starts on its own block
                writeln!(f)?;
                for row in self.0 {
                    // Collect the char Vec into a String for easy printing
                    let row_str: String = row.iter().collect();
                    writeln!(f, "    {}", row_str)?;
                }
                Ok(())
            }
        }

        f.debug_struct("CompressedGrid")
            .field("x_remap", &self.x_remap)
            .field("y_remap", &self.y_remap)
            // Wrap the grid in our helper to trigger the custom formatting
            .field("grid", &GridView(&self.grid))
            .finish()
    }
}
impl CompressedGrid {
    fn new(coordinates: &[Coordinate]) -> Self {
        if coordinates.is_empty() {
            return Self::default();
        }

        // Compress coordinates. Leave space in between so we can distinguish
        // #.#      ##
        // ...  and ##
        // #.#
        let mut x_remap = HashMap::new();
        coordinates
            .iter()
            .map(|c| c.x)
            .sorted()
            .unique()
            .enumerate()
            .for_each(|(idx, x)| {
                x_remap.insert(x, 2 * idx);
            });

        let mut y_remap = HashMap::new();
        coordinates
            .iter()
            .map(|c| c.y)
            .sorted()
            .unique()
            .enumerate()
            .for_each(|(idx, y)| {
                y_remap.insert(y, 2 * idx);
            });

        let max_y = *y_remap.values().max().expect("at least one y");
        let max_x = *x_remap.values().max().expect("at least one x");
        let mut grid = vec![vec!['.'; max_x + 1]; max_y + 1];

        // Add the vertices
        for c in coordinates {
            let (mapped_x, mapped_y) = (
                *x_remap.get(&c.x).expect("by construction"),
                *y_remap.get(&c.y).expect("by construction"),
            );
            println!("filling ({},{})", mapped_x, mapped_y);
            let loc = grid
                .get_mut(mapped_y)
                .expect("by construction")
                .get_mut(mapped_x)
                .expect("by construction");
            *loc = '#';
        }

        // Connect adjacent corners
        for (c1, c2) in coordinates.iter().circular_tuple_windows() {
            if c1.x == c2.x {
                let x = *x_remap.get(&c1.x).expect("by construction");
                let (y1, y2) = (
                    y_remap.get(&c1.y).expect("by construction"),
                    y_remap.get(&c2.y).expect("by construction"),
                );
                let (min_y, max_y) = (*y1.min(y2), *y1.max(y2));
                for y in min_y + 1..max_y {
                    grid[y][x] = 'X';
                }
            } else {
                let y = *y_remap.get(&c1.y).expect("by construction");
                let (x1, x2) = (
                    x_remap.get(&c1.x).expect("by construction"),
                    x_remap.get(&c2.x).expect("by construction"),
                );
                let (min_x, max_x) = (*x1.min(x2), *x1.max(x2));
                for x in min_x + 1..max_x {
                    grid[y][x] = 'X';
                }
            }
        }

        // Fill in the rest
        for y in 0..max_y + 1 {
            let mut fill = false;
            for x in 0..max_x + 1 {
                match grid[y][x] {
                    'X' | '#' => {
                        fill = !fill;
                    }
                    '.' => {
                        if fill {
                            grid[y][x] = 'X';
                        }
                    }
                    _ => unreachable!("no other chars in grid"),
                }
            }
        }

        dbg!(Self {
            x_remap,
            y_remap,
            grid,
        })
    }
}

fn valid_corners(a: &Coordinate, b: &Coordinate, compressed: &CompressedGrid) -> bool {
    let (min_x, max_x) = (a.x.min(b.x), a.x.max(b.x));
    let (min_y, max_y) = (a.y.min(b.y), a.y.max(b.y));

    let (min_x_remap, max_x_remap) = (compressed.x_remap[&min_x], compressed.x_remap[&max_x]);
    let (min_y_remap, max_y_remap) = (compressed.y_remap[&min_y], compressed.y_remap[&max_y]);

    for y in min_y_remap..=max_y_remap {
        for x in min_x_remap..=max_x_remap {
            match compressed.grid[y][x] {
                '.' => return false,
                _ => continue,
            }
        }
    }

    return true;
}

fn part2(coordinates: &[Coordinate]) -> Result<u64> {
    let compressed = CompressedGrid::new(coordinates);

    coordinates
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| valid_corners(a, b, &compressed).then_some(enclosed_area(a, b)))
        .max()
        .ok_or_else(|| anyhow!("no suitable rectangle exists"))
}

fn main() -> Result<()> {
    start_day(DAY);
    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read(INPUT_FILE)?;
    let input = parse_coordinates(BufReader::new(file.as_slice()))?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input)?;
    let p1_elapsed = p1_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_elapsed);

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input)?;
    let p2_elapsed = p2_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_elapsed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn parse_coordinates() {
        let result = super::parse_coordinates(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        let input = result.unwrap();
        assert_eq!(8, input.len());
    }

    #[test]
    fn part_1() {
        let expected = 50;
        let input =
            super::parse_coordinates(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 24;
        let input =
            super::parse_coordinates(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(&input);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn large_empty() {
        // Square with a big hole in the middle
        let coordinates = vec![
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 8, y: 0 },
            Coordinate { x: 8, y: 8 },
            Coordinate { x: 0, y: 8 },
            Coordinate { x: 0, y: 5 },
            Coordinate { x: 1, y: 5 },
            Coordinate { x: 1, y: 7 },
            Coordinate { x: 7, y: 7 },
            Coordinate { x: 7, y: 1 },
            Coordinate { x: 1, y: 1 },
            Coordinate { x: 1, y: 4 },
            Coordinate { x: 0, y: 4 },
        ];
        let result = part2(&coordinates);
        assert_eq!(result.unwrap(), 16)
    }
}
