use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as read_i32, newline};
use nom::multi::{many_till, separated_list1};
use nom::IResult;
use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Fold {
    X(i32),
    Y(i32),
}

/********************* Read input *********************/
fn read_dot(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, x) = read_i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = read_i32(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (x, y)))
}

fn read_fold(input: &str) -> nom::IResult<&str, Fold> {
    let (input, _) = tag("fold along ")(input)?;
    let (input, dir) = alt((tag("x="), tag("y=")))(input)?;
    let (input, line) = read_i32(input)?;

    match dir {
        "x=" => Ok((input, Fold::X(line))),
        "y=" => Ok((input, Fold::Y(line))),
        _ => panic!("Can't happen"),
    }
}

type Board = Vec<(i32, i32)>;
fn read(input: &str) -> nom::IResult<&str, (Board, Vec<Fold>)> {
    let (input, (dots, _)) = many_till(read_dot, newline)(input)?;
    let (input, folds) = separated_list1(newline, read_fold)(input)?;

    Ok((input, (dots, folds)))
}

/********************* Actual work *********************/

// Assuming fold always happens on halfway point
fn fold_map(dots: &mut Vec<(i32, i32)>, fold: Fold) {
    for (x, y) in dots.iter_mut() {
        match fold {
            Fold::X(f) => {
                if *x > f {
                    *x = (2 * f) - *x
                }
            }
            Fold::Y(f) => {
                if *y > f {
                    *y = (2 * f) - *y
                }
            }
        }
    }
    dots.sort_unstable();
    dots.dedup();
}

#[test]
fn test_fold() {
    {
        let mut v = vec![(0, 0), (20, 10)];
        fold_map(&mut v, Fold::X(10));
        assert_eq!(v, vec![(0, 0), (0, 10)]);
    }
    {
        let mut v = vec![(0, 0), (5, 10)];
        fold_map(&mut v, Fold::X(10));
        assert_eq!(v, vec![(0, 0), (5, 10)]);
    }
    {
        let mut v = vec![(0, 0), (20, 0)];
        fold_map(&mut v, Fold::X(10));
        assert_eq!(v, vec![(0, 0)]);
    }
}

// Assuming sorted
fn print_map(dots: &[(i32, i32)], x_size: i32, y_size: i32) {
    for y in 0..y_size {
        for x in 0..x_size {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let contents = fs::read_to_string("input/day13.txt").expect("Failed to read file");
    let (_, (mut dots, folds)) = read(&contents).expect("Failed to read instructions");

    dots.sort_unstable();

    let mut x_size = 0;
    let mut y_size = 0;
    for fold in folds {
        fold_map(&mut dots, fold);
        match fold {
            Fold::X(f) => x_size = f,
            Fold::Y(f) => y_size = f,
        }
    }

    print_map(&dots, x_size, y_size);
}
