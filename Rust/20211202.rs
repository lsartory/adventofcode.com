use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211202.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(Clone, Default)]
struct Record {
    direction: String,
    amount:    i64
}

fn parse_input(input: Vec<String>) -> Vec<Record> {
    let mut ret = Vec::new();
    for line in input {
        let mut rec: Record = Default::default();
        for (i, part) in line.split(' ').map(|x| x.trim().to_string()).enumerate() {
            if i == 0 {
                rec.direction = part;
            } else if let Ok(x) = part.parse() {
                rec.amount = x;
                ret.push(rec);
                break;
            }
        }
    }
    ret
}

/***********************************************/

fn part_1(input: &[Record]) {
    let mut pos = (0, 0);
    for i in input {
        match i.direction.as_str() {
            "forward" => { pos.0 += i.amount },
            "down"    => { pos.1 += i.amount },
            "up"      => { pos.1 -= i.amount },
                    _ => {}
        }
    }
    println!("Part 1: Horizontal position is {}, depth is {:6} → Product is {:10}", pos.0, pos.1, pos.0 * pos.1);
}

fn part_2(input: &[Record]) {
    let mut pos = (0, 0, 0);
    for i in input {
        match i.direction.as_str() {
            "forward" => { pos.0 += i.amount; pos.1 += pos.2 * i.amount },
            "down"    => { pos.2 += i.amount },
            "up"      => { pos.2 -= i.amount },
                    _ => {}
        }
    }
    println!("Part 2: Horizontal position is {}, depth is {:6} → Product is {:10}", pos.0, pos.1, pos.0 * pos.1);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
