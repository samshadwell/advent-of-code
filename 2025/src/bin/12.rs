use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use parse::parse;
use std::io::BufReader;
use std::ops::Range;
use std::time::Instant;

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

struct Shape {
    occupied_spaces: usize,
}

impl Shape {
    fn new(shape: Vec<Vec<bool>>) -> Self {
        Self {
            occupied_spaces: shape.iter().flatten().filter(|&&b| b).count(),
        }
    }
}

struct Region {
    length: usize,
    width: usize,
    requirements: Vec<usize>,
}

struct Input {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

mod parse {
    use super::{Input, Region, Shape};
    use anyhow::{Result, anyhow};
    use nom::Finish;
    use nom::IResult;
    use nom::Parser;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, newline, usize};
    use nom::combinator::value;
    use nom::multi::count;
    use nom::multi::many1;
    use nom::multi::separated_list1;
    use nom::sequence::delimited;
    use nom::sequence::terminated;
    use std::io::Read;

    pub fn parse<R: Read>(mut reader: R) -> Result<Input> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let input = &buf;
        let (remaining, shapes) = many1(delimited(shape_header, shape, newline))
            .parse(input)
            .map_err(|e| anyhow!("Shape parse error: {}", e))?;
        let (remaining, regions) = many1(region)
            .parse(remaining)
            .finish()
            .map_err(|e| anyhow!("Region parse error: {}", e))?;

        if !remaining.is_empty() {
            return Err(anyhow!("Failed to parse entire input"));
        }

        Ok(Input { shapes, regions })
    }

    fn shape_header(input: &str) -> IResult<&str, usize> {
        terminated(usize, tag(":\n")).parse(input)
    }

    fn shape(input: &str) -> IResult<&str, Shape> {
        let (input, shape) = count(
            terminated(
                count(alt((value(true, char('#')), value(false, char('.')))), 3),
                newline,
            ),
            3,
        )
        .parse(input)?;
        Ok((input, Shape::new(shape)))
    }

    fn region(input: &str) -> IResult<&str, Region> {
        let (input, length) = terminated(usize, char('x')).parse(input)?;
        let (input, width) = terminated(usize, tag(": ")).parse(input)?;
        let (input, requirements) =
            terminated(separated_list1(char(' '), usize), newline).parse(input)?;

        Ok((
            input,
            Region {
                length,
                width,
                requirements,
            },
        ))
    }
}

// True if there are enough 3x3s in the grid for all shapes to get one
fn region_definitely_possible(r: &Region) -> bool {
    let num_shapes: usize = r.requirements.iter().sum();
    let num_boxes = (r.length / 3) * (r.width / 3);
    num_shapes <= num_boxes
}

// True if there are at least enough empty spaces for the shapes to
// occupy (if we could cut the shapes up)
fn region_maybe_possible(r: &Region, shapes: &[Shape]) -> bool {
    let min_required_spaces: usize = r
        .requirements
        .iter()
        .zip(shapes)
        .map(|(n, shape)| n * shape.occupied_spaces)
        .sum();
    r.length * r.width >= min_required_spaces
}

fn part1(input: &Input) -> Result<Range<usize>> {
    let min = input
        .regions
        .iter()
        .filter(|r| region_definitely_possible(r))
        .count();
    let max = input
        .regions
        .iter()
        .filter(|r| region_maybe_possible(r, &input.shapes))
        .count();

    Ok(min..max)
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Parsing input ===");
    let parse_time = Instant::now();
    let file = std::fs::read(INPUT_FILE)?;
    let input = parse(BufReader::new(file.as_slice()))?;
    println!("Parsing time = {:.2?}\n", parse_time.elapsed());

    println!("=== Part 1 ===");
    let p1_time = Instant::now();
    let result = part1(&input)?;
    println!("Result in range: {} - {}", result.start, result.end);
    println!("Elapsed = {:.2?}", p1_time.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse as base_parse, part1};
    use std::io::BufReader;

    const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn parse() {
        let result = base_parse::parse(BufReader::new(TEST.as_bytes()));
        assert!(result.is_ok());
        let input = result.unwrap();
        assert_eq!(6, input.shapes.len());
        assert_eq!(3, input.regions.len());
    }

    #[test]
    fn part_1() {
        let expected = 0..3;
        let input = base_parse::parse(BufReader::new(TEST.as_bytes())).expect("parse succeeds");
        let result = part1(&input);
        assert_eq!(result.unwrap(), expected);
    }
}
