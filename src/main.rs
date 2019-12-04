use std::env;
use std::io::Error;

mod day01;
mod day02;
mod day03;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "day1" | "day01" => {
            println!("{}", day01::part1()?);
            println!("{}", day01::part2()?);
        }
        "day2" | "day02" => {
            println!("{}", day02::part1()?);
            println!("{}", day02::part2()?);
        }
        "day2_debug" => println!("{:?}", day02::print_values()?),

        "day3" | "day03" => {
            println!("{}", day03::part1()?);
            println!("{}", day03::part2()?);
        }
        _ => println!("{}", "invalid argument"),
    }
    Ok(())
}
