use adv_code_2025::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use good_lp::microlp;
use good_lp::{Solution, SolverModel, variable, variables};
use std::collections::{HashSet, VecDeque};
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
    joltage_requirements: Vec<u32>,
}

mod parse {
    use anyhow::{Result, anyhow};
    use nom::Finish;
    use nom::IResult;
    use nom::Parser;
    use nom::branch::alt;
    use nom::character::complete::{char, multispace0, u32, usize};
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

    fn joltage_requirements(input: &str) -> IResult<&str, Vec<u32>> {
        delimited(char('{'), separated_list1(char(','), u32), char('}')).parse(input)
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

fn min_p2(m: &Machine) -> Result<u32> {
    // Use an LP solver (microlp) to solve, my "best" normal solution was painfully slow
    let mut vars = variables!();

    // Each variable, i, corresponds to the number of times button i is pressed
    let counts: Vec<_> = (0..m.buttons.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    // Objective: Minimize sum of all counts (total button presses)
    let objective = counts.iter().sum::<good_lp::Expression>();
    let mut problem = vars.minimise(objective).using(microlp);

    // Constraints: In the end, we must get the required joltage vector
    for (jolt_idx, &target_val) in m.joltage_requirements.iter().enumerate() {
        let mut row_expr = good_lp::Expression::from(0);
        for (btn_idx, btn) in m.buttons.iter().enumerate() {
            if btn.toggled_lights.contains(&jolt_idx) {
                row_expr += counts.get(btn_idx).expect("button indeces are valid");
            }
        }

        problem.add_constraint(row_expr.eq(target_val));
    }

    let solution = problem
        .solve()
        .map_err(|e| anyhow!("Solver error: {}", e))?;

    let total_presses = solution.eval(counts.iter().sum::<good_lp::Expression>());
    // total_presses is float type, but we have constrained to be integer above
    // so conversion should succeed
    Ok(total_presses as u32)
}

fn part2(machines: &[Machine]) -> Result<u32> {
    machines.iter().map(min_p2).sum()
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
