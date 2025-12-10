use adv_code_2025::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::BufReader;
use std::time::Instant;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Light {
    Off,
    On,
}

impl Light {
    fn toggle(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Button {
    toggled_lights: Vec<usize>,
}

struct Machine {
    goal_state: Vec<Light>,
    buttons: Vec<Button>,
    joltage_requirements: Vec<u64>,
}

mod parse {
    use anyhow::{Result, anyhow};
    use nom::Finish;
    use nom::IResult;
    use nom::Parser;
    use nom::branch::alt;
    use nom::character::complete::{char, multispace0, u64, usize};
    use nom::combinator::value;
    use nom::multi::{many0, many1, separated_list1};
    use nom::sequence::{delimited, terminated};
    use std::io::BufRead;

    fn light(input: &str) -> IResult<&str, super::Light> {
        alt((
            value(super::Light::Off, char('.')),
            value(super::Light::On, char('#')),
        ))
        .parse(input)
    }

    fn goal_state(input: &str) -> IResult<&str, Vec<super::Light>> {
        delimited(char('['), many0(light), char(']')).parse(input)
    }

    fn button(input: &str) -> IResult<&str, super::Button> {
        let (input, toggled_lights) =
            delimited(char('('), separated_list1(char(','), usize), char(')')).parse(input)?;
        Ok((input, super::Button { toggled_lights }))
    }

    fn joltage_requirements(input: &str) -> IResult<&str, Vec<u64>> {
        delimited(char('{'), separated_list1(char(','), u64), char('}')).parse(input)
    }

    fn machine(input: &str) -> IResult<&str, super::Machine> {
        let (input, goal_state) = terminated(&goal_state, multispace0).parse(input)?;
        let (input, buttons) = many1(terminated(&button, multispace0)).parse(input)?;
        let (input, joltage_requirements) = joltage_requirements(input)?;
        Ok((
            input,
            super::Machine {
                goal_state,
                buttons,
                joltage_requirements,
            },
        ))
    }

    pub fn parse<R: BufRead>(reader: R) -> Result<Vec<super::Machine>> {
        reader
            .lines()
            .map(|l| {
                let (_, m) = machine
                    .parse(&l?)
                    .finish()
                    .map_err(|e| anyhow!(e.to_string()))?;
                Ok(m)
            })
            .collect()
    }
}

fn min_p1_button_pushes(m: &Machine) -> Result<u64> {
    let mut seen: HashSet<Vec<Light>> = HashSet::new();
    let mut queue: VecDeque<(Vec<Light>, u64)> = VecDeque::new();
    let initial = vec![Light::Off; m.goal_state.len()];

    queue.push_back((initial, 0));
    while let Some((state, so_far)) = queue.pop_front() {
        if state == m.goal_state {
            return Ok(so_far);
        }

        for b in &m.buttons {
            let mut next = state.clone();
            for to_toggle in &b.toggled_lights {
                let light = next.get_mut(*to_toggle).expect("button valid");
                *light = light.toggle();
            }
            if !seen.contains(&next) {
                seen.insert(next.clone());
                queue.push_back((next, so_far + 1));
            }
        }
    }

    Err(anyhow!(
        "unable to reach goal state with any number of button presses"
    ))
}

fn part1(machines: &[Machine]) -> Result<u64> {
    machines.iter().map(min_p1_button_pushes).sum()
}

fn min_p2_button_pushes(m: &Machine) -> Result<u64> {
    // Optimization: Sort buttons to try "biggest" impact first
    let mut sorted_buttons: Vec<&Button> = m.buttons.iter().collect();
    sorted_buttons.sort_by(|a, b| b.toggled_lights.len().cmp(&a.toggled_lights.len()));

    let start = vec![0; m.joltage_requirements.len()];
    let mut heap = BinaryHeap::new();

    let max_cardinality = sorted_buttons
        .first()
        .map(|b| b.toggled_lights.len())
        .expect("at least one button will exist") as u64;

    #[derive(Eq, PartialEq)]
    struct State {
        f_score: u64,
        cost: u64,
        node: Vec<u64>,
        min_idx: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            // Reversed because BinaryHeap is a MaxHeap, we want MinHeap behavior
            other.f_score.cmp(&self.f_score)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let heuristic_distance = |state: &Vec<u64>| -> u64 {
        let mut distance = 0;
        let mut max_single = 0;
        for (i, n) in state.iter().enumerate() {
            let req_n = m
                .joltage_requirements
                .get(i)
                .expect("state len matches joltage len");
            if n > req_n {
                // No number of button presses make joltage go down
                return u64::MAX;
            }
            let dist = req_n - n;
            distance += dist;
            max_single = max_single.max(dist);
        }
        distance.div_ceil(max_cardinality).max(max_single)
    };

    heap.push(State {
        f_score: heuristic_distance(&start),
        cost: 0,
        node: start.clone(),
        min_idx: 0,
    });
    let mut g_score = HashMap::new();
    g_score.insert((start, 0), 0);

    while let Some(State {
        cost,
        node,
        min_idx,
        ..
    }) = heap.pop()
    {
        if node == m.joltage_requirements {
            return Ok(cost);
        }

        if let Some(&best_g) = g_score.get(&(node.clone(), min_idx))
            && cost > best_g
        {
            continue;
        }

        for (i, b) in sorted_buttons.iter().enumerate().skip(min_idx) {
            let mut neighbor = node.clone();
            for to_incr in &b.toggled_lights {
                let n = neighbor.get_mut(*to_incr).expect("button valid");
                *n += 1;
            }

            let new_cost = cost + 1;
            let h = heuristic_distance(&neighbor);
            if h == u64::MAX {
                continue;
            }
            let new_f = new_cost + h;
            let state_key = (neighbor.clone(), i);
            if new_cost < *g_score.get(&state_key).unwrap_or(&u64::MAX) {
                g_score.insert(state_key, new_cost);
                heap.push(State {
                    f_score: new_f,
                    cost: new_cost,
                    node: neighbor,
                    min_idx: i,
                });
            }
        }
    }

    Err(anyhow!("no combination of button presses exists"))
}

fn part2(machines: &[Machine]) -> Result<u64> {
    machines.iter().map(|m| dbg!(min_p2_button_pushes(m))).sum()
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let input = std::fs::read(INPUT_FILE)?;
    let machines = parse::parse(BufReader::new(input.as_slice()))?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&machines)?;
    let p1_elapsed = p1_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}\n", p1_elapsed);

    println!("=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&machines)?;
    let p2_elapsed = p2_time.elapsed();
    println!("Result = {}", result);
    println!("Elapsed = {:.2?}", p2_elapsed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn parse_test() {
        let machines = parse::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        assert_eq!(3, machines.len())
    }

    #[test]
    fn part_1() {
        let expected = 7;
        let machines = parse::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&machines);
        assert_eq!(result.unwrap(), expected)
    }

    #[test]
    fn part_2() {
        let expected = 33;
        let machines = parse::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part2(&machines);
        assert_eq!(result.unwrap(), expected)
    }
}
