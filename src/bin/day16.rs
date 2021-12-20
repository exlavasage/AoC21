use nom::bits::complete::take;
use nom::multi::many_m_n;
use std::fs;

fn read_version(input: (&[u8], usize)) -> nom::IResult<(&[u8], usize), usize> {
    take(3usize)(input)
}

fn read_type(input: (&[u8], usize)) -> nom::IResult<(&[u8], usize), u8> {
    take(3usize)(input)
}

fn read_literal(input: (&[u8], usize)) -> nom::IResult<(&[u8], usize), usize> {
    let mut data = input;
    let mut literal = 0usize;
    let mut cont = 1;
    while cont == 1 {
        let (input, c) = take(1usize)(data)?;
        let (input, nibble): ((&[u8], usize), usize) = take(4usize)(input)?;
        cont = c;
        literal <<= 4;
        literal += nibble;
        data = input;
    }

    Ok((data, literal))
}

fn calc_length(input: (&[u8], usize)) -> usize {
    input.0.len() * 8 - input.1
}

fn read_subpackets(input: (&[u8], usize)) -> nom::IResult<(&[u8], usize), Vec<(usize, usize)>> {
    let (input, sub): ((&[u8], usize), usize) = take(1usize)(input)?;
    if sub == 0usize {
        let (mut input, length): ((&[u8], usize), usize) = take(15usize)(input)?;

        // NOTE: No nice way to use take to split input
        let goal = calc_length(input) - length;
        let mut results = Vec::new();
        while goal < calc_length(input) {
            let (data, value) = read(input)?;
            input = data;
            results.push(value);
        }
        Ok((input, results))
    } else {
        let (input, length): ((&[u8], usize), usize) = take(11usize)(input)?;
        many_m_n(length, length, read)(input)
    }
}

fn read(input: (&[u8], usize)) -> nom::IResult<(&[u8], usize), (usize, usize)> {
    let (input, ver) = read_version(input)?;
    let (input, ty) = read_type(input)?;
    match ty {
        4 /* Literal */ => {
            let (input, value) = read_literal(input)?;
            Ok((input, (value, ver)))
        }
        _ => {
            let (input, values) = read_subpackets(input)?;
            let mut ver_sum = ver;
            for (_, version) in &values {
                ver_sum += version;
            }
            match ty {
                0 => Ok((input, (values.iter().fold(0, |acc, x| acc + x.0), ver_sum))),
                1 => Ok((input, (values.iter().fold(1, |acc, x| acc * x.0), ver_sum))),
                2 => Ok((input, (*values.iter().map(|(v, _)| v).min().unwrap(), ver_sum))),
                3 => Ok((input, (*values.iter().map(|(v, _)| v).max().unwrap(), ver_sum))),
                5 => Ok((input, (if values[0].0 > values[1].0 { 1} else { 0}, ver_sum))),
                6 => Ok((input, (if values[0].0 < values[1].0 { 1} else { 0}, ver_sum))),
                7 => Ok((input, (if values[0].0 == values[1].0 { 1} else { 0}, ver_sum))),
                _ => Ok((input, (values[0].0, ver_sum))),
            }
        }
    }
}

#[test]
fn test() {
    let (_, value) = read((&hex::decode("D2FE28").unwrap(), 0)).unwrap();
    assert_eq!(value.0, 2021usize);
    assert_eq!(value.1, 6);

    let (_, value) = read((&hex::decode("38006F45291200").unwrap(), 0)).unwrap();
    //assert_eq!(value.0, 10);
    assert_eq!(value.1, 9);

    let (_, value) = read((&hex::decode("EE00D40C823060").unwrap(), 0)).unwrap();
    //assert_eq!(value.0, 1);
    assert_eq!(value.1, 14);

    let (_, value) = read((&hex::decode("C200B40A82").unwrap(), 0)).unwrap();
    assert_eq!(value.0, 3);

    let (_, value) = read((&hex::decode("04005AC33890").unwrap(), 0)).unwrap();
    assert_eq!(value.0, 54);

    let (_, value) = read((&hex::decode("9C005AC2F8F0").unwrap(), 0)).unwrap();
    assert_eq!(value.0, 0);

    let (_, value) = read((&hex::decode("9C0141080250320F1802104A08").unwrap(), 0)).unwrap();
    assert_eq!(value.0, 1);
}

fn main() {
    let mut contents = fs::read_to_string("input/day16.txt").expect("Failed to read file");
    contents.pop(); // remove \n
    let bytes = match hex::decode(&contents) {
        Ok(bytes) => bytes,
        Err(_) => {
            contents.push('0');
            hex::decode(&contents).expect("Failed to parse hex")
        }
    };

    let (_, value) = read((&bytes, 0usize)).expect("Failed to parse bytes");
    println!("Value: {:?}", value);
}
