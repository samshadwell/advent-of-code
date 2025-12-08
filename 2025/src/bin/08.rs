use adv_code_2025::start_day;
use anyhow::{Context, Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{Ord, PartialOrd, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::io::{BufRead, BufReader};
use std::sync::LazyLock;
use std::time::Instant;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    id: u64,
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

    fn parse_str(id: u64, s: &str) -> Result<Self> {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^(\d+),(\d+),(\d+)$").expect("valid regex"));
        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("invalid point format: {}", s))?;
        let x = caps[1].parse().context("parsing x")?;
        let y = caps[2].parse().context("parsing y")?;
        let z = caps[3].parse().context("parsing z")?;

        Ok(Self { id, x, y, z })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pair<'a, 'b> {
    a: &'a Point,
    b: &'b Point,
}

// Order pairs of points by the distaince between their constituent points
impl Ord for Pair<'_, '_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.a.square_distance(self.b)).cmp(&other.a.square_distance(other.b))
    }
}

impl PartialOrd for Pair<'_, '_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct ConnectedGroups<'a> {
    point_to_group: HashMap<&'a Point, u64>,
    group_to_points: HashMap<u64, Vec<&'a Point>>,
}

impl<'a> ConnectedGroups<'a> {
    fn new() -> Self {
        ConnectedGroups {
            point_to_group: HashMap::new(),
            group_to_points: HashMap::new(),
        }
    }

    // Returns the number of elements in the resulting group
    fn connect(&mut self, a: &'a Point, b: &'a Point) -> usize {
        let group_a = *self.point_to_group.get(a).unwrap_or(&a.id);
        let group_b = *self.point_to_group.get(b).unwrap_or(&b.id);
        if group_a == group_b {
            return self
                .group_to_points
                .get(&group_a)
                .map(|g| g.len())
                .unwrap_or(1);
        }

        // Ensure both groups exist in group_to_points before merging
        self.group_to_points
            .entry(group_a)
            .or_insert_with(|| vec![a]);
        self.group_to_points
            .entry(group_b)
            .or_insert_with(|| vec![b]);

        let new_group_id = group_a.min(group_b);
        let old_group_id = group_a.max(group_b);

        let old_points = self
            .group_to_points
            .remove(&old_group_id)
            .expect("group guaranteed to exist");

        for point in &old_points {
            self.point_to_group.insert(point, new_group_id);
        }

        let new_group = self
            .group_to_points
            .get_mut(&new_group_id)
            .expect("should exist");
        new_group.extend(old_points);
        new_group.len()
    }

    fn part1_score(&self) -> u64 {
        self.group_to_points
            .values()
            .map(|v| v.len() as u64)
            .k_largest(3)
            .product()
    }
}

fn read_points<R: BufRead>(reader: R) -> Result<Vec<Point>> {
    reader
        .lines()
        .enumerate()
        .map(|(i, l)| Point::parse_str(i as u64, &l?))
        .collect()
}

fn make_pairs_min_heap<'a>(points: &'a [Point]) -> BinaryHeap<Reverse<Pair<'a, 'a>>> {
    let mut heap = BinaryHeap::new();
    for (i, a) in points.iter().enumerate() {
        for b in points.get(i + 1..).unwrap_or_default() {
            let pair = Pair { a, b };
            // Reverse to make min-heap
            heap.push(Reverse(pair));
        }
    }
    heap
}

fn part1<R: BufRead>(reader: R, num_connections: usize) -> Result<u64> {
    let points = read_points(reader)?;
    let mut heap = make_pairs_min_heap(&points);

    let mut connected_groups = ConnectedGroups::new();
    for _ in 0..num_connections {
        let to_connect = heap
            .pop()
            .ok_or_else(|| anyhow!("ran out of elements to connect"))?
            .0;
        connected_groups.connect(to_connect.a, to_connect.b);
    }
    Ok(connected_groups.part1_score())
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let points = read_points(reader)?;
    let mut heap = make_pairs_min_heap(&points);

    let mut connected_groups = ConnectedGroups::new();
    let num_points = points.len();
    loop {
        let Pair { a, b } = heap
            .pop()
            .ok_or_else(|| {
                anyhow!("programmer error, should always have sufficient elements to fully connect")
            })?
            .0;
        let group_size = connected_groups.connect(a, b);
        if group_size == num_points {
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
