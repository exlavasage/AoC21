use nom::character::complete::{alpha1, i32 as read_i32, newline, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;
use std::fs;

use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

/********************* Read input *********************/
impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

fn read_direction(input: &str) -> IResult<&str, Direction> {
    map_res(alpha1, Direction::from_str)(input)
}

#[test]
fn test_read_direction() {
    assert_eq!(read_direction("up"), Ok(("", Direction::Up)));
    assert_eq!(read_direction("down 42"), Ok((" 42", Direction::Down)));
    assert_eq!(read_direction("forward"), Ok(("", Direction::Forward)));
}

fn read_line(input: &str) -> IResult<&str, (Direction, i32)> {
    let (input, dir) = read_direction(input)?;
    let (input, _) = space1(input)?;
    let (input, digit) = read_i32(input)?;

    Ok((input, (dir, digit)))
}

#[test]
fn test_read_line() {
    assert_eq!(read_line("down 42"), Ok(("", (Direction::Down, 42))));
    assert_eq!(
        read_line("forward 1241"),
        Ok(("", (Direction::Forward, 1241)))
    );
    assert_eq!(read_line("up 1\n"), Ok(("\n", (Direction::Up, 1))));
}

fn read(input: &str) -> nom::IResult<&str, Vec<(Direction, i32)>> {
    separated_list1(newline, read_line)(input)
}

/********************* Actual work *********************/
fn follow_path(instructions: &[(Direction, i32)]) -> i32 {
    let mut depth = 0;
    let mut dist = 0;
    for (dir, length) in instructions {
        match dir {
            Direction::Down => depth += length,
            Direction::Up => depth -= length,
            Direction::Forward => dist += length,
        }
    }

    depth * dist
}

fn follow_aim(instructions: &[(Direction, i32)]) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut dist = 0;
    for (dir, length) in instructions {
        match dir {
            Direction::Down => aim += length,
            Direction::Up => aim -= length,
            Direction::Forward => {
                dist += length;
                depth += length * aim;
            }
        }
    }

    depth * dist
}

fn main() {
    let contents = fs::read_to_string("input/day2.txt").expect("Failed to read file");
    let (_, instructions) = read(&contents).expect("Failed to read instructions");

    println!("Distance: {}", follow_path(&instructions));
    println!("Aim: {}", follow_aim(&instructions));
}
