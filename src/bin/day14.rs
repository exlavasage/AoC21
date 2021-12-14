use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, newline};
use nom::multi::separated_list1;
use std::collections::HashMap;
use std::fs;

/********************* Read input *********************/
fn read_transform(input: &str) -> nom::IResult<&str, ((char, char), char)> {
    let (input, a) = anychar(input)?;
    let (input, b) = anychar(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, right) = anychar(input)?;

    Ok((input, ((a, b), right)))
}

type Transforms = Vec<((char, char), char)>;
fn read(input: &str) -> nom::IResult<&str, (&str, Transforms)> {
    let (input, poly) = alpha1(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, transforms) = separated_list1(newline, read_transform)(input)?;

    Ok((input, (poly, transforms)))
}

/********************* Actual work *********************/
fn calc_diff(pair_counts: &HashMap<(char, char), isize>) -> f64 {
    let mut counts = HashMap::new();
    for (&(a, b), &count) in pair_counts {
        if let Some(v) = counts.insert(a, count) {
            *counts.get_mut(&a).unwrap() += v;
        }
        if let Some(v) = counts.insert(b, count) {
            *counts.get_mut(&b).unwrap() += v;
        }
    }

    (*counts.values().max().unwrap() as f64 / 2.0f64).ceil()
        - (*counts.values().min().unwrap() as f64 / 2.0f64).ceil()
}

#[test]
fn test_calc_diff() {
    assert_eq!(
        calc_diff(&HashMap::from([(('A', 'B'), 1), (('B', 'B'), 1),])),
        1f64
    );
}

fn apply_n(
    n: isize,
    pair_counts: &mut HashMap<(char, char), isize>,
    transforms: &HashMap<(char, char), char>,
) {
    for _ in 0..n {
        let mut new_counts = HashMap::new();
        for (&(a, b), &c) in transforms {
            if let Some(&count) = pair_counts.get(&(a, b)) {
                if let Some(v) = new_counts.insert((a, c), count) {
                    *new_counts.get_mut(&(a, c)).unwrap() += v;
                }
                if let Some(v) = new_counts.insert((c, b), count) {
                    *new_counts.get_mut(&(c, b)).unwrap() += v;
                }
            }
        }
        *pair_counts = new_counts;
    }
}

#[test]
fn test() {
    let content = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    let (_, (poly, trans)) = read(content).expect("Failed to read instructions");
    let mut transforms = HashMap::new();
    for (left, right) in trans {
        transforms.insert(left, right);
    }
    let mut pair_counts = poly_to_counts(poly);

    apply_n(1, &mut pair_counts, &transforms);
    assert_eq!(pair_counts, poly_to_counts("NCNBCHB"));
    apply_n(1, &mut pair_counts, &transforms);
    assert_eq!(pair_counts, poly_to_counts("NBCCNBBBCBHCB"));
    apply_n(1, &mut pair_counts, &transforms);
    assert_eq!(pair_counts, poly_to_counts("NBBBCNCCNBBNBNBBCHBHHBCHB"));
    apply_n(1, &mut pair_counts, &transforms);
    assert_eq!(
        pair_counts,
        poly_to_counts("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")
    );
}

fn poly_to_counts(poly: &str) -> HashMap<(char, char), isize> {
    let mut pair_counts: HashMap<(char, char), isize> = HashMap::new();
    for sl in poly.as_bytes().windows(2) {
        if let Some(c) = pair_counts.get_mut(&(sl[0].into(), sl[1].into())) {
            *c += 1
        } else {
            pair_counts.insert((sl[0].into(), sl[1].into()), 1);
        }
    }

    pair_counts
}

fn main() {
    let contents = fs::read_to_string("input/day14.txt").expect("Failed to read file");
    let (_, (poly, trans)) = read(&contents).expect("Failed to read instructions");

    let mut transforms = HashMap::new();
    for (left, right) in trans {
        transforms.insert(left, right);
    }

    let mut pair_counts = poly_to_counts(poly);
    apply_n(1, &mut pair_counts, &transforms);
    apply_n(9, &mut pair_counts, &transforms);
    println!("Count: {}", calc_diff(&pair_counts));
    apply_n(30, &mut pair_counts, &transforms);
    println!("Count: {}", calc_diff(&pair_counts));
}
