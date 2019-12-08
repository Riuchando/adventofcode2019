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
    let instructions: Vec<i32> = instructions
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // before running the program, replace position 1 with the value 12 and
    // replace position 2 with the value 2.
    let diagnostic_code = solve_opcode(instructions.clone(), 1);
    Ok(diagnostic_code as u64)
}

pub fn part2() -> Result<u64, Error> {
    let f = File::open("resources/day05.txt")?;
    let mut f = BufReader::new(f);
    let mut buf = String::new();
    f.read_line(&mut buf)?;
    let instructions: Vec<&str> = buf.split(',').collect();
    let instructions: Vec<i32> = instructions
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let diagnostic_code = solve_big_opcode(instructions.clone(), 5);
    Ok(diagnostic_code as u64)
}

fn solve_opcode(instructions: Vec<i32>, input: u32) -> i32 {
    let mut instructions = instructions;
    let mut output = 0;
    let mut opcode = instructions[0];
    let mut index: usize = 0;
    while opcode != 99 {
        if opcode == 1 {
            let first_index: usize = instructions[index + 1] as usize;
            let second_index: usize = instructions[index + 2] as usize;
            let last_index: usize = instructions[index + 3] as usize;

            instructions[last_index] = instructions[first_index] + instructions[second_index];
            // println!("{} = {} + {} ", instructions[index + 3] as usize, instructions[first_index], instructions[second_index]);
            index += 4;
        } else if opcode == 2 {
            let first_index: usize = instructions[index + 1] as usize;
            let second_index: usize = instructions[index + 2] as usize;
            let last_index: usize = instructions[index + 3] as usize;

            instructions[last_index] = instructions[first_index] * instructions[second_index];
            // println!("{} = {} * {} ", instructions[index + 3] as usize, instructions[first_index], instructions[second_index]);
            index += 4;
        } else if opcode == 3 {
            let first_index: usize = instructions[index + 1] as usize;
            instructions[first_index] = input as i32;
            // println!("{}", instructions[first_index]);
            index += 2;
        } else if opcode == 4 {
            let first_index: usize = instructions[index + 1] as usize;
            output = instructions[first_index];
            // println!("output {}, index {}", output, first_index);
            index += 2;
        } else {
            // let first_index: usize = instructions[index + 1] as usize;
            let opcode_parse = parse_digits(opcode as i32);
            println!("{:?}", opcode_parse);
            match opcode_parse.4 {
                1 => {
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };

                    let second_value = if opcode_parse.1 == 0 {
                        instructions[instructions[index + 2] as usize]
                    } else {
                        instructions[index + 2]
                    };
                    let last_index: usize = instructions[index + 3] as usize;
                    instructions[last_index] = first_value + second_value;

                    // println!("{} = {} + {} ", instructions[index + 3] as usize, first_value, second_value);
                    index += 4;
                }
                2 => {
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };

                    let second_value = if opcode_parse.1 == 0 {
                        instructions[instructions[index + 2] as usize]
                    } else {
                        instructions[index + 2]
                    };
                    let last_index: usize = instructions[index + 3] as usize;
                    instructions[last_index] = first_value * second_value;
                    // println!("{} = {} * {}", instructions[index + 3] as usize, first_value, second_value);
                    index += 4;
                }
                3 => {
                    println!("failed parse {:?}", opcode_parse);
                    index += 2;
                }
                4 => {
                    // println!("{:?}", opcode_parse);
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };

                    // println!("{:?} {}",  opcode_parse, first_value);
                    // println!("output {}, index {}", output, instructions[index + 1]);
                    output = first_value;
                    index += 2;
                }
                _ => {
                    println!("{:?}", opcode_parse);
                }
            }
        }

        opcode = instructions[index];
    }
    output
}

fn parse_digits(number: i32) -> (i32, i32, i32, i32, i32) {
    (
        (number / 10_000) % 10,
        (number / 1_000) % 10,
        (number / 100) % 10,
        (number / 10) % 10,
        number % 10,
    )
}

fn solve_big_opcode(instructions: Vec<i32>, input: u32) -> i32 {
    let mut instructions = instructions;
    // instructions[1] = noun;
    // instructions[2] = verb;
    let mut output = 0;
    let mut opcode = instructions[0];
    let mut index: usize = 0;
    while opcode != 99 {
        if opcode == 1 {
            let first_index: usize = instructions[index + 1] as usize;
            let second_index: usize = instructions[index + 2] as usize;
            let last_index: usize = instructions[index + 3] as usize;

            instructions[last_index] = instructions[first_index] + instructions[second_index];
            // println!("{} = {} + {} ", instructions[index + 3] as usize, instructions[first_index], instructions[second_index]);
            index += 4;
        } else if opcode == 2 {
            let first_index: usize = instructions[index + 1] as usize;
            let second_index: usize = instructions[index + 2] as usize;
            let last_index: usize = instructions[index + 3] as usize;

            instructions[last_index] = instructions[first_index] * instructions[second_index];
            // println!("{} = {} * {} ", instructions[index + 3] as usize, instructions[first_index], instructions[second_index]);
            index += 4;
        } else if opcode == 3 {
            let first_index: usize = instructions[index + 1] as usize;
            instructions[first_index] = input as i32;
            // println!("{}", instructions[first_index]);
            index += 2;
        } else if opcode == 4 {
            let first_index: usize = instructions[index + 1] as usize;
            output = instructions[first_index];
            // println!("output {}, index {}", output, first_index);
            index += 2;
        } else if opcode == 5 {
            let first_index: usize = instructions[index + 1] as usize;
            if instructions[first_index] != 0 {
                let second_index: usize = instructions[index + 2] as usize;
                index = instructions[second_index] as usize;
            } else {
                index += 3;
            }
        } else if opcode == 6 {
            let first_index: usize = instructions[index + 1] as usize;
            if instructions[first_index] == 0 {
                let second_index: usize = instructions[index + 2] as usize;
                index = instructions[second_index] as usize;
            } else {
                index += 3;
            }
        } else if opcode == 7 {
            let first_index: usize = instructions[index + 1] as usize;
            let second_index: usize = instructions[index + 2] as usize;
            let third_index: usize = instructions[index + 3] as usize;
            if instructions[first_index] < instructions[second_index] {
                instructions[third_index] = 1;
            } else {
                instructions[third_index] = 0;
            }
            index += 4;
        } else if opcode == 8 {
            let first_index: usize = instructions[index + 1] as usize;
            let second_index: usize = instructions[index + 2] as usize;
            let third_index: usize = instructions[index + 3] as usize;
            if instructions[first_index] == instructions[second_index] {
                instructions[third_index] = 1;
            } else {
                instructions[third_index] = 0;
            }
            index += 4;
        } else {
            // let first_index: usize = instructions[index + 1] as usize;
            let opcode_parse = parse_digits(opcode as i32);
            println!("{:?}", opcode_parse);
            match opcode_parse.4 {
                1 => {
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };

                    let second_value = if opcode_parse.1 == 0 {
                        instructions[instructions[index + 2] as usize]
                    } else {
                        instructions[index + 2]
                    };
                    let last_index: usize = instructions[index + 3] as usize;
                    instructions[last_index] = first_value + second_value;

                    // println!("{} = {} + {} ", instructions[index + 3] as usize, first_value, second_value);
                    index += 4;
                }
                2 => {
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };

                    let second_value = if opcode_parse.1 == 0 {
                        instructions[instructions[index + 2] as usize]
                    } else {
                        instructions[index + 2]
                    };
                    let last_index: usize = instructions[index + 3] as usize;
                    instructions[last_index] = first_value * second_value;
                    // println!("{} = {} * {}", instructions[index + 3] as usize, first_value, second_value);
                    index += 4;
                }
                3 => {
                    println!("failed parse {:?}", opcode_parse);
                    index += 2;
                }
                4 => {
                    // println!("{:?}", opcode_parse);
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };

                    // println!("{:?} {}",  opcode_parse, first_value);
                    // println!("output {}, index {}", output, instructions[index + 1]);
                    output = first_value;
                    index += 2;
                }
                5 => {
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };
                    let second_value = if opcode_parse.1 == 0 {
                        instructions[instructions[index + 2] as usize]
                    } else {
                        instructions[index + 2]
                    };

                    if first_value != 0 {
                        index = second_value as usize;
                    } else {
                        index += 3;
                    }
                }
                6 => {
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };
                    let second_value = if opcode_parse.1 == 0 {
                        instructions[instructions[index + 2] as usize]
                    } else {
                        instructions[index + 2]
                    };
                    if first_value == 0 {
                        index = second_value as usize;
                    } else {
                        index += 3;
                    }
                }
                7 => {
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };
                    let second_value = if opcode_parse.1 == 0 {
                        instructions[instructions[index + 2] as usize]
                    } else {
                        instructions[index + 2]
                    };
                    let third_value = if opcode_parse.0 == 0 {
                        instructions[instructions[index + 3] as usize]
                    } else {
                        instructions[index + 3]
                    };
                    let third_index: usize = instructions[index + 3] as usize;
                    if first_value < second_value {
                        instructions[third_index] = 1;
                    } else {
                        instructions[third_index] = 0;
                    }
                    index += 4;
                }
                8 => {
                    let first_value = if opcode_parse.2 == 0 {
                        instructions[instructions[index + 1] as usize]
                    } else {
                        instructions[index + 1]
                    };
                    let second_value = if opcode_parse.1 == 0 {
                        instructions[instructions[index + 2] as usize]
                    } else {
                        instructions[index + 2]
                    };
                    let third_index: usize = instructions[index + 3] as usize;

                    if first_value == second_value {
                        instructions[third_index] = 1;
                    } else {
                        instructions[third_index] = 0;
                    }
                    index += 4;
                }
                _ => {
                    println!("{:?}", opcode_parse);
                }
            }
        }

        opcode = instructions[index];
    }
    output
}
