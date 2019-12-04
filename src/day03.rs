use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1() -> Result<u64, Error> {
    let f = File::open("resources/day03.txt")?;
    let mut f = BufReader::new(f);
    let mut buf_1 = String::new();
    // this should be fine
    let size = (30000, 30000);
    let mut grid = vec![vec![(0u8, 0u8); size.0 as usize]; size.1 as usize];
    let mut intersections: Vec<(usize, usize)> = Vec::new();
    f.read_line(&mut buf_1)?;
    let default_cursor = (size.0/2, size.1/2);
    let wire_1: Vec<&str> = buf_1[..buf_1.len() - 1].split(',').collect();
    let mut cursor = default_cursor;
    for instruction in wire_1 {
        let direction = instruction.chars().next().unwrap();
        let amount = instruction[1..].parse::<u32>().map_err(|_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("couldn't parse {}", instruction),
            )
        })?;
        match direction {
            'U' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].0 = 1;
                    cursor.0 = cursor.0 + 1;
                }
            }
            'R' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].0 = 1;
                    cursor.1 = cursor.1 + 1;
                }
            }

            'L' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].0 = 1;
                    cursor.1 = cursor.1 - 1;
                }
            }

            'D' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].0 = 1;
                    cursor.0 = cursor.0 - 1;
                }
            }
            _ => {
                println!("invalid instruction {}", instruction);
                ()
            }
        }
    }
    let mut buf_2 = String::new();
    f.read_line(&mut buf_2)?;
    let wire_2: Vec<&str> = buf_2.split(',').collect();
    cursor = default_cursor;
    for instruction in wire_2 {
        let direction = instruction.chars().next().unwrap();
        let amount = instruction[1..].parse::<u32>().map_err(|_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("couldn't parse {}", instruction),
            )
        })?;
        match direction {
            'U' => {
                for _ in 0..amount {
                    if grid[cursor.0][cursor.1].0 == 1 && cursor != default_cursor {
                        intersections.push(cursor);
                    }
                    grid[cursor.0][cursor.1].1 = 1;

                    cursor.0 = cursor.0 + 1;
                }
            }
            'R' => {
                for _ in 0..amount {
                    if grid[cursor.0][cursor.1].0 == 1 && cursor != default_cursor {
                        intersections.push(cursor);
                    }
                    grid[cursor.0][cursor.1].1 = 1;
                    cursor.1 = cursor.1 + 1;
                }
            }

            'L' => {
                for _ in 0..amount {
                    if grid[cursor.0][cursor.1].0 == 1 && cursor != default_cursor {
                        intersections.push(cursor);
                    }
                    grid[cursor.0][cursor.1].1 = 1;
                    cursor.1 = cursor.1 - 1;
                }
            }

            'D' => {
                for _ in 0..amount {
                    if grid[cursor.0][cursor.1].0 == 1 && cursor != default_cursor {
                        intersections.push(cursor);
                    }
                    grid[cursor.0][cursor.1].1 = 1;
                    cursor.0 = cursor.0 - 1;
                }
            }
            _ => {
                println!("invalid instruction {}", instruction);
                ()
            }
        }
    }
    // println!("{:?}", intersections);
    // let mut min_distance = 9999999;
    // for intersection in intersections {
    //     let x_dist = if intersection.0 > default_cursor.0 {
    //         intersection.0 - default_cursor.0
    //     } else {
    //         default_cursor.0 - intersection.0
    //     };
    //     let y_dist = if intersection.1 > default_cursor.1 {
    //         intersection.1 - default_cursor.1
    //     } else {
    //         default_cursor.1 - intersection.1
    //     };
    //     if x_dist + y_dist < min_distance {
    //         min_distance = x_dist + y_dist;
    //     }
    // }

    Ok( match intersections.iter().map(|(x,y)| if *x > default_cursor.0{
            x - default_cursor.0
        } else {
            default_cursor.0 - x
        }+ if *y > default_cursor.1 {
            y - default_cursor.1
        } else {
            default_cursor.1 - y
        }).min() {
            Some(distance) => distance,
            None => 0,
        } as u64)
}

pub fn part2() -> Result<u64, Error> {
    let f = File::open("resources/day03.txt")?;
    let mut f = BufReader::new(f);
    let mut buf_1 = String::new();
    // this should be fine
    let size = (30000, 30000);
    let mut grid = vec![vec![(0u32, 0u32); size.0 as usize]; size.1 as usize];
    let mut intersections: Vec<(u32, u32)> = Vec::new();
    f.read_line(&mut buf_1)?;
    let default_cursor = (size.0 / 2, size.1 / 2);
    let wire_1: Vec<&str> = buf_1[..buf_1.len() - 1].split(',').collect();
    let mut cursor = default_cursor;
    let mut step_count = 0;
    for instruction in wire_1 {
        let direction = instruction.chars().next().unwrap();
        let amount = instruction[1..].parse::<u32>().map_err(|_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("couldn't parse {}", instruction),
            )
        })?;
        match direction {
            'U' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].0 = step_count;
                    cursor.0 = cursor.0 + 1;
                    step_count = step_count + 1;
                }
            }
            'R' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].0 = step_count;
                    cursor.1 = cursor.1 + 1;
                    step_count = step_count + 1;
                }
            }

            'L' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].0 = step_count;
                    cursor.1 = cursor.1 - 1;
                    step_count = step_count + 1;
                }
            }

            'D' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].0 = step_count;
                    cursor.0 = cursor.0 - 1;
                    step_count = step_count + 1;
                }
            }
            _ => {
                println!("invalid instruction {}", instruction);
                ()
            }
        }
    }
    let mut buf_2 = String::new();
    f.read_line(&mut buf_2)?;
    let wire_2: Vec<&str> = buf_2.split(',').collect();
    cursor = default_cursor;
    step_count = 0;
    for instruction in wire_2 {
        let direction = instruction.chars().next().unwrap();
        let amount = instruction[1..].parse::<u32>().map_err(|_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("couldn't parse {}", instruction),
            )
        })?;
        match direction {
            'U' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].1 = step_count;
                    step_count += 1;
                    if grid[cursor.0][cursor.1].0 != 0 && cursor != default_cursor {
                        intersections.push(grid[cursor.0][cursor.1]);
                    }

                    cursor.0 = cursor.0 + 1;
                }
            }
            'R' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].1 = step_count;
                    step_count += 1;
                    if grid[cursor.0][cursor.1].0 != 0 && cursor != default_cursor {
                        intersections.push(grid[cursor.0][cursor.1]);
                    }

                    cursor.1 = cursor.1 + 1;
                }
            }

            'L' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].1 = step_count;
                    step_count += 1;
                    if grid[cursor.0][cursor.1].0 != 0 && cursor != default_cursor {
                        intersections.push(grid[cursor.0][cursor.1]);
                    }

                    cursor.1 = cursor.1 - 1;
                }
            }

            'D' => {
                for _ in 0..amount {
                    grid[cursor.0][cursor.1].1 = step_count;
                    step_count += 1;
                    if grid[cursor.0][cursor.1].0 != 0 && cursor != default_cursor {
                        intersections.push(grid[cursor.0][cursor.1]);
                    }

                    cursor.0 = cursor.0 - 1;
                }
            }
            _ => {
                println!("invalid instruction {}", instruction);
                ()
            }
        }
    }

    Ok(match intersections
        .iter()
        .map(|(wire_1_count, wire_2_count)| wire_1_count + wire_2_count)
        .min()
    {
        Some(x) => x,
        None => 0,
    } as u64)
}
