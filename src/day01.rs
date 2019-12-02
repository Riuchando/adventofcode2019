use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1() -> Result<u32, Error> {
    let f = File::open("resources/day01.txt")?;
    let f = BufReader::new(f);

    f.lines()
        .try_fold(0u32, |acc, line| match parse_line(line) {
            Ok(number) => Ok(acc + number / 3 - 2),
            Err(err) => Err(err),
        })
}

fn parse_line(line: Result<String, Error>) -> Result<u32, Error> {
    line?
        .parse::<u32>()
        .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
}

fn recursive_fuel(fuel: u32) -> u32 {
    let mut fuel = fuel;
    let mut sum = 0u32;
    while fuel / 3 > 2 {
        fuel = fuel / 3 - 2;
        sum += fuel;
    }
    return sum;
}

pub fn part2() -> Result<u32, Error> {
    let f = File::open("resources/day01.txt")?;
    let f = BufReader::new(f);

    f.lines()
        .try_fold(0u32, |acc, line| match parse_line(line) {
            Ok(number) => Ok(acc + recursive_fuel(number)),
            Err(err) => Err(err),
        })
}
