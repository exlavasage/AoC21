use nom::character::complete::{newline, one_of};
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
use nom::IResult;
use std::fs;

static mut INFINITE_PIXEL: u8 = 0;

fn read_point(input: &str) -> IResult<&str, u8> {
    map(one_of("#."), |c| if c == '.' { 0 } else { 1 })(input)
}

fn read_line(input: &str) -> IResult<&str, Vec<u8>> {
    many0(read_point)(input)
}

fn read(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list0(newline, read_line)(input)
}

fn read_map(input: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let (_, map) = read(input).expect("Failed to read input");
    (map[0].clone(), map[2..].to_vec())
}

fn get_point(map: &[Vec<u8>], x: isize, y: isize) -> u8 {
    if x >= 0 && y >= 0 && (x as usize) < map[0].len() && (y as usize) < map.len() {
        map[y as usize][x as usize]
    } else {
        unsafe { INFINITE_PIXEL }
    }
}

fn points_to_value(map: &[Vec<u8>], x: isize, y: isize) -> usize {
    let mut value = 0usize;
    for a in (y - 1)..(y + 2) {
        for b in (x - 1)..(x + 2) {
            value <<= 1;
            value += get_point(map, b, a) as usize;
        }
    }
    value as usize
}

fn enhance(ima: &[u8], map: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut new_map = Vec::new();
    for y in -1isize..((map.len() + 2) as isize) {
        let mut row = Vec::new();
        for x in -1isize..((map[0].len() + 2) as isize) {
            let value = points_to_value(map, x, y);
            row.push(ima[value]);
        }
        new_map.push(row);
    }
    unsafe {
        INFINITE_PIXEL = if INFINITE_PIXEL == 1 {
            ima[ima.len() - 1]
        } else {
            ima[0]
        };
    }
    new_map
}

fn count(map: &[Vec<u8>]) -> usize {
    let mut n: usize = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            n += map[y][x] as usize;
        }
    }
    n
}

fn draw(map: &[Vec<u8>]) {
    for row in map {
        for v in row {
            print!("{}", if *v == 0 { '.' } else { '#' });
        }
        println!();
    }
}

fn draw_window(map: &[Vec<u8>], x: isize, y: isize) {
    for a in (y - 1)..(y + 2) {
        for b in (x - 1)..(x + 2) {
            print!("{}", if get_point(map, b, a) == 0 { '.' } else { '#' });
        }
        println!();
    }
}

#[test]
fn test() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    let (ima, mut map) = read_map(input);
    assert_eq!(ima.len(), 512);
    assert_eq!(map.len(), 5);
    assert_eq!(map[0].len(), 5);
    assert_eq!(count(&map), 10);

    draw(&map);
    let value = points_to_value(&map, 2, 2);
    assert_eq!(value, 34);
    assert_eq!(ima[value], 1);

    draw_window(&map, 4, 1);
    let value = points_to_value(&map, 4, 1);
    assert_eq!(value, 258);

    for _ in 0..2 {
        map = enhance(&ima, &map);
        draw(&map);
    }
    assert_eq!(count(&map), 35);
}

fn main() {
    let mut contents = fs::read_to_string("input/day20.txt").expect("Failed to read file");
    contents.pop();
    let (ima, mut map) = read_map(&contents);

    for _ in 0..2 {
        map = enhance(&ima, &map);
    }
    println!("Count: {}", count(&map));

    for _ in 2..50 {
        map = enhance(&ima, &map);
    }
    println!("Count: {}", count(&map));
}
