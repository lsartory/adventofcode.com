use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201215.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<u64> {
    let mut ret = Vec::new();
    for line in input {
        for x in line.split(',') {
            match x.parse::<u64>() { Ok(x) => ret.push(x), _ => {} }
        }
    }
    ret
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());

    let mut mem = HashMap::new();
    let mut turn: u64 = 1;
    let mut next = 0;
    for i in input {
        println!("{} → {}", turn, i);
        next = match mem.insert(i, turn) { Some(x) => turn - x, _ => 0 };
        turn += 1;
    }
    println!(".\n.\n.");
    while turn < 2020 {
        next = match mem.insert(next, turn) { Some(x) => turn - x, _ => 0 };
        turn += 1;
    }
    println!("{} → {}", turn, next);
    println!(".\n.\n.");
    while turn < 30000000 {
        next = match mem.insert(next, turn) { Some(x) => turn - x, _ => 0 };
        turn += 1;
    }
    println!("{} → {}", turn, next);
}
