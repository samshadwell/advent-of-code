use adv_code_2025::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{Ord, PartialOrd, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Instant;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn square_distance(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2)
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^(\d+),(\d+),(\d+)$").expect("valid regex"));
        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid point format: {}", s))?;
        let x = caps[1].parse().context("parsing x")?;
        let y = caps[2].parse().context("parsing y")?;
        let z = caps[3].parse().context("parsing z")?;

        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pair<'a> {
    a: &'a Point,
    b: &'a Point,
}

// Order pairs of points by the distance between their constituent points
impl Ord for Pair<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.a.square_distance(self.b)).cmp(&other.a.square_distance(other.b))
    }
}

impl PartialOrd for Pair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct DisjointSetForest<'a, T> {
    parents: HashMap<&'a T, &'a T>,
    sizes: HashMap<&'a T, u64>,
}

// See: https://en.wikipedia.org/wiki/Disjoint-set_data_structure#Operations
impl<'a, T: Eq + Hash> DisjointSetForest<'a, T> {
    fn new() -> Self {
        DisjointSetForest {
            parents: HashMap::new(),
            sizes: HashMap::new(),
        }
    }

    fn make_set(&mut self, x: &'a T) -> Result<()> {
        if !self.parents.contains_key(x) {
            self.parents.insert(x, x);
            self.sizes.insert(x, 1);
            Ok(())
        } else {
            Err(anyhow!("set with given key already exists"))
        }
    }

    fn _find(&mut self, x: &'a T) -> Result<&'a T> {
        let mut curr = x;
        loop {
            let parent = *self
                .parents
                .get(curr)
                .ok_or_else(|| anyhow!("x not found"))?;
            if parent == curr {
                return Ok(curr);
            }

            let grandparent = *self.parents.get(parent).expect("parents should be valid");
            self.parents.insert(curr, grandparent);
            curr = parent;
        }
    }

    fn union(&mut self, x: &'a T, y: &'a T) -> Result<u64> {
        let x_root = self._find(x)?;
        let y_root = self._find(y)?;

        if x_root == y_root {
            return Ok(*self.sizes.get(x_root).expect("x is known to be in forest"));
        }

        let x_size = *self.sizes.get(x_root).expect("x has known size");
        let y_size = *self.sizes.get(y_root).expect("y has known size");

        let (smaller, larger) = if x_size < y_size {
            (x_root, y_root)
        } else {
            (y_root, x_root)
        };
        self.parents.insert(larger, smaller);
        self.sizes.insert(smaller, x_size + y_size);
        self.sizes.remove(larger);

        Ok(x_size + y_size)
    }

    fn part1_score(&self) -> u64 {
        self.sizes.values().k_largest(3).product()
    }
}

fn read_points<R: BufRead>(reader: R) -> Result<Vec<Point>> {
    reader.lines().map(|l| Point::from_str(&l?)).collect()
}

fn make_pairs_min_heap<'a>(points: &'a [Point]) -> BinaryHeap<Reverse<Pair<'a>>> {
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Reverse(Pair { a, b }))
        .collect()
}

fn part1<R: BufRead>(reader: R, num_connections: usize) -> Result<u64> {
    let points = read_points(reader)?;
    let mut heap = make_pairs_min_heap(&points);

    let mut set_forest = DisjointSetForest::new();
    for point in &points {
        set_forest.make_set(point)?;
    }
    for _ in 0..num_connections {
        let to_connect = heap
            .pop()
            .ok_or_else(|| anyhow!("ran out of elements to connect"))?
            .0;
        set_forest.union(to_connect.a, to_connect.b)?;
    }
    Ok(set_forest.part1_score())
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let points = read_points(reader)?;
    let mut heap = make_pairs_min_heap(&points);

    let mut set_forest = DisjointSetForest::new();
    for point in &points {
        set_forest.make_set(point)?;
    }
    let num_points = points.len();
    loop {
        let Pair { a, b } = heap
            .pop()
            .ok_or_else(|| {
                anyhow!("programmer error, should always have sufficient elements to fully connect")
            })?
            .0;
        let group_size = set_forest.union(a, b)?;
        if group_size == num_points as u64 {
            return Ok(a.x * b.x);
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read(INPUT_FILE)?;
    start_day(DAY);

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part1(input_file, 1000)?;
    let p1_elapsed = p1_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p1_elapsed);

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let input_file = BufReader::new(input.as_slice());
    let result = part2(input_file)?;
    let p2_elapsed = p2_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_elapsed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn part_1() {
        let expected = 40;
        let result = part1(BufReader::new(TEST.as_bytes()), 10);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 25272;
        let result = part2(BufReader::new(TEST.as_bytes()));
        assert_eq!(result.unwrap(), expected)
    }
}
