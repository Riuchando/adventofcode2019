use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1() -> Result<u32, Error> {
    // let f = File::open("resources/day06.txt")?;
    // let f = BufReader::new(f);
    // let mut children: HashMap<String, HashSet<String>> = HashMap::new();
    // let mut parents: HashMap<String, String> = HashMap::new();
    // for line in f.lines() {
    //     let line_split: Vec<&str> = line?.
    //         .parse::<String>()
    //         .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))?
    //         .split(')')
    //         .collect();
    //         let parent : String = line_split[0].to_owned().clone();
    //         let child: String = line_split[1].to_owned()clone();
    //         let default: HashSet<String> = HashSet::new();
    //         default.insert(child);
    //     let child_ref = children
    //         .entry(parent)
    //         .or_insert(default).union(&default);
            
    // }
    Ok(0)
}
