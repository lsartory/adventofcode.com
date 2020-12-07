use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201202.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

struct Record {
    left:     u16,
    right:    u16,
    ch:       char,
    password: String
}

fn parse_input(input: Vec<String>) -> Vec<Record> {
    let mut ret = Vec::new();
    for line in input.iter() {
        let parts = line.split(&['-', ' ', ':'][..]).map(|x| x.trim()).filter(|x| !x.is_empty()).collect::<Vec<_>>();
        if parts.len() != 4 {
            continue;
        }
        ret.push(Record {
            left:     match parts[0].parse::<u16>()  { Ok(x) => x, _ =>  0  },
            right:    match parts[1].parse::<u16>()  { Ok(x) => x, _ =>  0  },
            ch:       match parts[2].parse::<char>() { Ok(x) => x, _ => '?' },
            password: parts[3].to_string()
        });
    }
    ret
}

/***********************************************/

fn part_1(input: &Vec<Record>) {
    println!("Valid passwords (part 1): {}", input.iter().map(|r| { let count = r.password.matches(r.ch).count() as u16; if (count >= r.left) && (count <= r.right) { 1 } else { 0 } }).sum::<u32>());
}

fn part_2(input: &Vec<Record>) {
    println!("Valid passwords (part 2): {}", input.iter().map(|r| { let password: Vec<char> = r.password.chars().collect(); if (password[r.left as usize - 1] == r.ch) != (password[r.right as usize - 1] == r.ch) { 1 } else { 0 } }).sum::<u32>());
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
