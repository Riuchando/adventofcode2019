use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;


pub fn part1() -> Result<u64, Error> {
    let f = File::open("resources/day05.txt")?;
    let mut f = BufReader::new(f);
    let mut buf = String::new();
    f.read_line(&mut buf)?;
    let instructions: Vec<&str> = buf.split(',').collect();
    let instructions: Vec<u32> = instructions
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // before running the program, replace position 1 with the value 12 and
    // replace position 2 with the value 2.
    let (head, _, _) = solve_opcode(instructions.clone(), 12, 2);
    Ok(head as u64)
    // Ok(0)
}
fn solve_opcode(instructions: Vec<u32>, noun: u32, verb: u32) -> (u32, u32, u32) {
    let mut instructions = instructions;
    instructions[1] = noun;
    instructions[2] = verb;
    let mut opcode = instructions[0];
    let mut index: usize = 0;
    while opcode != 99 {
        match opcode {
            1 => {
                let first_index: usize = instructions[index + 1] as usize;
                let second_index: usize = instructions[index + 2] as usize;
                let last_index: usize = instructions[index + 3] as usize;

                instructions[last_index] = instructions[first_index] + instructions[second_index];
                index += 4;
            }
            2 => {
                let first_index: usize = instructions[index + 1] as usize;
                let second_index: usize = instructions[index + 2] as usize;
                let last_index: usize = instructions[index + 3] as usize;

                instructions[last_index] = instructions[first_index] * instructions[second_index];
                index += 4;
            }
            3 => {

            }
            _ => (),
        };
        
        opcode = instructions[index];
    }
    (instructions[0], instructions[1], instructions[2])
}
