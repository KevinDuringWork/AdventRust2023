use nom::IResult;
use std::error::Error;

use nom::branch::{alt};
use nom::bytes::complete::{tag};
use nom::character::complete::{alpha1};
use nom::combinator::value; 
use nom::multi::many1;
use std::fs::read_to_string;

fn parse_lit(input: &str) -> IResult<&str, &str> {
    alt((
        value("1", tag("one")),
        value("2", tag("two")),
        value("3", tag("three")),
        value("4", tag("four")),
        value("5", tag("five")),
        value("6", tag("six")),
        value("7", tag("seven")),
        value("8", tag("eight")),
        value("9", tag("nine")),
    ))(input)
}

fn parse_single(input: &str) -> IResult<&str, &str> {
    // shaky..
    if input.len() > 0 {
        return Ok((&input[1..input.len()], &input[0..1]))
    }

    alpha1(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((parse_lit, parse_single)))(input)
}

fn parse_line(input: &str) -> Result<i64, Box<dyn Error + '_>> {
    let (_, output) = parse_input(input)?;

    let d: String = output
        .into_iter()
        .flat_map(|c| c.parse::<i64>())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    let c: Vec<_> = d.chars().collect();
    let s = format!("{}{}", c.first().unwrap(), c.last().unwrap()).parse::<i64>()?;
    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<i64> = read_to_string("test2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .map(|x| parse_line(x.as_str()).unwrap())
        .collect();

    dbg!(lines.iter().sum::<i64>());
    Ok(())
}
