use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, usize};
use nom::combinator::all_consuming;
use nom::{Finish, IResult, Parser};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Default)]
struct WeightedGraph {
    ids: HashMap<String, usize>,
    weights: Vec<Vec<usize>>,
}

impl WeightedGraph {
    fn ensure_id(&mut self, n: &str) -> usize {
        if let Some(&id) = self.ids.get(n) {
            id
        } else {
            let next_id = self.ids.len();
            self.ids.insert(n.to_string(), next_id);
            for w in &mut self.weights {
                w.push(0);
            }
            self.weights.push(vec![0; next_id + 1]);
            next_id
        }
    }

    fn update_weight(&mut self, n: usize, m: usize, weight: usize) {
        if let Some(cell) = self.weights.get_mut(n).and_then(|ws| ws.get_mut(m)) {
            *cell = weight;
        } else {
            unreachable!("for valid ids n, m, should be unreachable");
        }

        if let Some(cell) = self.weights.get_mut(m).and_then(|ws| ws.get_mut(n)) {
            *cell = weight;
        } else {
            unreachable!("for valid ids n, m, should be unreachable");
        }
    }

    fn add_edge(&mut self, n: &str, m: &str, weight: usize) {
        let (n_id, m_id) = (self.ensure_id(n), self.ensure_id(m));
        self.update_weight(n_id, m_id, weight);
    }

    fn num_nodes(&self) -> usize {
        self.ids.len()
    }
}

fn parse_line(s: &str) -> IResult<&str, (&str, &str, usize)> {
    all_consuming((alpha1, tag(" to "), alpha1, tag(" = "), usize))
        .parse(s)
        .map(|(res, (n, _, m, _, w))| (res, (n, m, w)))
}

fn parse<R: BufRead>(reader: R) -> Result<WeightedGraph> {
    let mut graph = WeightedGraph::default();
    for res_l in reader.lines() {
        let line = res_l.map_err(|e| anyhow!("error reading line {e}"))?;
        let (_, (n, m, w)) = parse_line(line.as_str())
            .finish()
            .map_err(|e| anyhow!("parse error {e}"))?;
        graph.add_edge(n, m, w);
    }
    Ok(graph)
}

fn route_weights_iter(input: &WeightedGraph) -> impl Iterator<Item = usize> {
    let n = input.num_nodes();
    (0..n).permutations(n).map(|order| {
        order
            .iter()
            .tuple_windows()
            .map(|(&n, &m)| {
                input
                    .weights
                    .get(n)
                    .expect("exists by construction")
                    .get(m)
                    .expect("exists by construction")
            })
            .sum()
    })
}

fn part1(input: &WeightedGraph) -> usize {
    if input.num_nodes() == 0 {
        return 0;
    }

    route_weights_iter(input)
        .min()
        .expect("at least one permutation, for non-empty graph")
}

fn part2(input: &WeightedGraph) -> usize {
    if input.num_nodes() == 0 {
        return 0;
    }

    route_weights_iter(input)
        .max()
        .expect("at least one permutation, for non-empty graph")
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

    const TEST: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";

    #[test]
    fn parse() {
        let result = super::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        assert_eq!(3, result.unwrap().weights.len());
    }

    #[test]
    fn part_1() {
        let expected = 605;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result, expected)
    }

    #[test]
    fn part_2() {
        let expected = 982;
        let input = super::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(&input);
        assert_eq!(result, expected)
    }
}
