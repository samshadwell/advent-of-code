use adv_code_2015::start_day;
use anyhow::{Result, anyhow};
use const_format::concatcp;
use std::collections::HashMap;
use std::time::Instant;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

enum GateRef {
    Literal(u16),
    Reference(String),
}

enum LogicGate {
    Wire(GateRef),
    And(GateRef, GateRef),
    LShift(GateRef, u32),
    Not(GateRef),
    Or(GateRef, GateRef),
    RShift(GateRef, u32),
}

struct Circuit {
    parts: HashMap<String, LogicGate>,
}

impl Circuit {
    fn value_inner(&self, g: &GateRef, cache: &mut HashMap<String, u16>) -> Result<u16> {
        match g {
            GateRef::Literal(v) => Ok(*v),
            GateRef::Reference(g) => {
                if let Some(&val) = cache.get(g) {
                    return Ok(val);
                }

                if let Some(gate) = self.parts.get(g) {
                    let val = match gate {
                        LogicGate::Wire(h) => self.value_inner(h, cache)?,
                        LogicGate::And(l, r) => {
                            let (l_val, r_val) =
                                (self.value_inner(l, cache)?, self.value_inner(r, cache)?);
                            l_val & r_val
                        }
                        LogicGate::LShift(h, amount) => {
                            let h_val = self.value_inner(h, cache)?;
                            h_val.wrapping_shl(*amount)
                        }
                        LogicGate::Not(h) => {
                            let h_val = self.value_inner(h, cache)?;
                            !h_val
                        }
                        LogicGate::Or(l, r) => {
                            let (l_val, r_val) =
                                (self.value_inner(l, cache)?, self.value_inner(r, cache)?);
                            l_val | r_val
                        }
                        LogicGate::RShift(h, amount) => {
                            let h_val = self.value_inner(h, cache)?;
                            h_val.wrapping_shr(*amount)
                        }
                    };
                    cache.insert(g.clone(), val);
                    Ok(val)
                } else {
                    Err(anyhow!("Circuit incomplete, reference {g} not found"))
                }
            }
        }
    }

    fn value(&self, g: &str) -> Result<u16> {
        let mut cache = HashMap::new();
        self.value_inner(&GateRef::Reference(g.to_string()), &mut cache)
    }

    fn update(&mut self, id: &str, g: LogicGate) -> Option<LogicGate> {
        self.parts.insert(id.to_string(), g)
    }
}

mod parse {
    use super::GateRef;
    use super::LogicGate;
    use anyhow::{Result, anyhow};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, multispace0, newline, u16, u32};
    use nom::combinator::{all_consuming, map};
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, terminated};
    use nom::{Finish, IResult, Parser};

    fn gate_ref(s: &str) -> IResult<&str, GateRef> {
        alt((
            map(u16, GateRef::Literal),
            map(alpha1, |s: &str| GateRef::Reference(s.to_string())),
        ))
        .parse(s)
    }

    fn parse_line(s: &str) -> IResult<&str, (String, LogicGate)> {
        map(
            (
                alt((
                    map(
                        separated_pair(gate_ref, tag(" AND "), gate_ref),
                        |(a, b)| LogicGate::And(a, b),
                    ),
                    map(
                        separated_pair(gate_ref, tag(" LSHIFT "), u32),
                        |(g, amt)| LogicGate::LShift(g, amt),
                    ),
                    map(preceded(tag("NOT "), gate_ref), LogicGate::Not),
                    map(separated_pair(gate_ref, tag(" OR "), gate_ref), |(a, b)| {
                        LogicGate::Or(a, b)
                    }),
                    map(
                        separated_pair(gate_ref, tag(" RSHIFT "), u32),
                        |(g, amt)| LogicGate::RShift(g, amt),
                    ),
                    map(gate_ref, LogicGate::Wire),
                )),
                tag(" -> "),
                alpha1,
            ),
            |(gate, _, g)| (g.to_string(), gate),
        )
        .parse(s)
    }

    pub fn parse(input: &str) -> Result<super::Circuit> {
        let (_, parts) = all_consuming(terminated(
            separated_list1(newline, parse_line),
            multispace0,
        ))
        .parse(input)
        .finish()
        .map_err(|e| anyhow!("parsing error: {e:?}"))?;
        Ok(super::Circuit {
            parts: parts.into_iter().collect(),
        })
    }
}

fn part1(input: &Circuit) -> Result<u16> {
    input.value("a")
}

fn part2(input: &mut Circuit) -> Result<u16> {
    let new_b = input.value("a")?;
    input.update("b", LogicGate::Wire(GateRef::Literal(new_b)));
    input.value("a")
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let input_data = std::fs::read_to_string(INPUT_FILE)?;
    let mut input = parse::parse(&input_data)?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&mut input)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse::parse;

    const TEST: &str = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
y -> z
1 AND 0 -> q
";

    #[test]
    fn test_parse() {
        let result = parse(TEST);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().parts.len(), 10);
    }

    #[test]
    fn test_value() {
        let circuit = parse(TEST).expect("parse succeeds");
        assert_eq!(circuit.value("d").expect("succeeds"), 72);
        assert_eq!(circuit.value("e").expect("succeeds"), 507);
        assert_eq!(circuit.value("f").expect("succeeds"), 492);
        assert_eq!(circuit.value("g").expect("succeeds"), 114);
        assert_eq!(circuit.value("h").expect("succeeds"), 65412);
        assert_eq!(circuit.value("i").expect("succeeds"), 65079);
        assert_eq!(circuit.value("x").expect("succeeds"), 123);
        assert_eq!(circuit.value("y").expect("succeeds"), 456);
        assert_eq!(circuit.value("z").expect("succeeds"), 456);
        assert_eq!(circuit.value("q").expect("succeeds"), 0);

        assert!(circuit.value("foo").is_err());
    }
}
