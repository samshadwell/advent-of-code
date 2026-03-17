use adv_code_2015::start_day;
use anyhow::{Context, Result};
use const_format::concatcp;
use json::JsonValue;
use regex::Regex;
use std::{sync::LazyLock, time::Instant};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn part1(input: &str) -> i32 {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"-?\d+").expect("regex parses successfully"));

    RE.find_iter(input)
        .map(|m| {
            m.as_str()
                .parse::<i32>()
                .expect("all matching str parse successfully")
        })
        .sum()
}

fn sum_non_red(jv: &JsonValue) -> Result<i64> {
    match jv {
        JsonValue::Null | JsonValue::Short(_) | JsonValue::String(_) | JsonValue::Boolean(_) => {
            Ok(0)
        }
        JsonValue::Number(n) => n
            .as_fixed_point_i64(0)
            .with_context(|| "received NaN number"),
        JsonValue::Array(v) => v.iter().map(sum_non_red).sum(),
        JsonValue::Object(o) => {
            if o.iter().any(|(_, v)| v == "red") {
                Ok(0)
            } else {
                o.iter().map(|(_, v)| sum_non_red(v)).sum()
            }
        }
    }
}

fn part2(input: &str) -> Result<i64> {
    let jv = json::parse(input).with_context(|| "input did not parse into JsonValue")?;
    sum_non_red(&jv)
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let input = std::fs::read_to_string(INPUT_FILE)?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input);
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    println!("\n=== Part 2 ===");
    let p2_time = Instant::now();
    let result = part2(&input)?;
    println!("Result = {result}");
    println!("Elapsed = {:.2?}", p2_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(6, part1("[1,2,3]"));
        assert_eq!(6, part1(r#"{"a":2,"b":4}"#));

        assert_eq!(3, part1("[[[3]]]"));
        assert_eq!(3, part1(r#"{"a":{"b":4},"c":-1}"#));

        assert_eq!(0, part1(r#"{"a":[-1,1]}"#));
        assert_eq!(0, part1(r#"[-1,{"a":1}]"#));
        assert_eq!(0, part1("[]"));
        assert_eq!(0, part1("{}"));
        assert_eq!(0, part1(""));
    }

    #[test]
    fn part_2() {
        assert_eq!(6, part2(r#"[1,2,3]"#).unwrap());
        assert_eq!(4, part2(r#"[1,{"c":"red","b":2},3]"#).unwrap());
        assert_eq!(0, part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap());
        assert_eq!(6, part2(r#"[1,"red",5]"#).unwrap());
    }
}
