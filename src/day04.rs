use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
// use std::io::ErrorKind;
use itertools::Itertools;

pub fn part1() -> Result<u32, Error> {
    let f = File::open("resources/day04.txt")?;
    let mut f = BufReader::new(f);
    let mut buf = String::new();
    f.read_line(&mut buf)?;
    let range: Vec<u32> = buf.split("-").map(|x| x.parse::<u32>().unwrap()).collect();
    // println!("{:?}", range);
    // println!("{} {} {}",adjacency(113456), adjacency(123789), ascending(223450));
    // println!("{} {} {} {}",adjacency(113456), adjacency(123789), ascending(223450), ascending(123789));

    Ok((range[0]..range[1])
        .filter(|number| adjacency(*number) && ascending(*number))
        .count() as u32)
}

fn adjacency(number: u32) -> bool {
    let first_digit = number / 100_000;
    let second_digit = (number / 10_000) % 10;
    let third_digit = (number / 1_000) % 10;
    let fourth_digit = (number / 100) % 10;
    let fifth_digit = (number / 10) % 10;
    let sixth_digit = number % 10;

    return first_digit == second_digit
        || second_digit == third_digit
        || third_digit == fourth_digit
        || fourth_digit == fifth_digit
        || fifth_digit == sixth_digit;
}

fn ascending(number: u32) -> bool {
    let first_digit = number / 100_000;
    let second_digit = (number / 10_000) % 10;
    let third_digit = (number / 1_000) % 10;
    let fourth_digit = (number / 100) % 10;
    let fifth_digit = (number / 10) % 10;
    let sixth_digit = number % 10;
    return first_digit <= second_digit
        && second_digit <= third_digit
        && third_digit <= fourth_digit
        && fourth_digit <= fifth_digit
        && fifth_digit <= sixth_digit;
}

fn two_group_adjacency(number: u32) -> bool {
    vec![
        number / 100_000,
        (number / 10_000) % 10,
        (number / 1_000) % 10,
        (number / 100) % 10,
        (number / 10) % 10,
        number % 10,
    ]
    .into_iter()
    .group_by(|x| *x)
    .into_iter()
    .map(|(_, group)| group.count())
    .filter(|count| *count == 2)
    .count()
        > 0
}

pub fn part2() -> Result<u32, Error> {
    let f = File::open("resources/day04.txt")?;
    let mut f = BufReader::new(f);
    let mut buf = String::new();
    f.read_line(&mut buf)?;
    let range: Vec<u32> = buf.split("-").map(|x| x.parse::<u32>().unwrap()).collect();

    Ok((range[0]..range[1])
        .filter(|number| adjacency(*number) && ascending(*number) && two_group_adjacency(*number))
        .count() as u32)
}
