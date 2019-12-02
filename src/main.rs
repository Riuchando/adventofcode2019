use std::env;
use std::io::Error;

mod day01;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "day1" | "day01" => {
            println!("{}", day01::part1()?);
            println!("{}", day01::part2()?);
        }
        _ => println!("{}", "invalid argument"),
    }
    Ok(())
}
