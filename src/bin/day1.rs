use nom::character::complete::{i32 as read_i32, newline};
use nom::multi::separated_list1;
use nom::IResult;
use std::fs;

/********************* Read input *********************/
fn read(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(newline, read_i32)(input)
}

/********************* Actual work *********************/
fn diff_depths(depths: &[i32]) -> isize {
    let mut inc_depths = 0;
    let mut last_depth = None;
    for depth in depths {
        if let Some(last) = last_depth {
            if last < depth {
                inc_depths += 1;
            }
        }
        last_depth = Some(depth)
    }
    inc_depths
}

fn diff_windows(depths: &[i32]) -> isize {
    let sl1 = &mut depths[..depths.len() - 3].iter();
    let sl2 = &mut depths[3..].iter();

    let mut inc = 0;
    for depth1 in sl1 {
        let depth2 = sl2.next().unwrap();
        if depth1 < depth2 {
            inc += 1;
        }
    }
    inc
}

fn main() {
    let contents = fs::read_to_string("input/day1/input.txt").expect("Failed to read file");
    let (_, depths) = read(&contents).expect("Failed to read depths");

    println!("Diff depths: {}", diff_depths(&depths));
    println!("Diff windows: {}", diff_windows(&depths));
}
