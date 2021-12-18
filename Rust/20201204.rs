use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201204.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(Default, PartialEq)]
struct Record {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

fn parse_input(input: Vec<String>) -> Vec<Record> {
    let mut ret = Vec::new();
    let mut record: Record = Default::default();
    for line in input {
        if !line.is_empty() {
            for k in line.split(' ') {
                let pair: Vec<&str> = k.split(':').collect();
                if pair.len() == 2 {
                    match pair[0] {
                        "byr" => record.byr = match pair[1].trim().parse() { Ok(x) => x, _ => 0 },
                        "iyr" => record.iyr = match pair[1].trim().parse() { Ok(x) => x, _ => 0 },
                        "eyr" => record.eyr = match pair[1].trim().parse() { Ok(x) => x, _ => 0 },
                        "hgt" => record.hgt = match pair[1].trim().parse() { Ok(x) => x, _ => "".to_string() },
                        "hcl" => record.hcl = match pair[1].trim().parse() { Ok(x) => x, _ => "".to_string() },
                        "ecl" => record.ecl = match pair[1].trim().parse() { Ok(x) => x, _ => "".to_string() },
                        "pid" => record.pid = match pair[1].trim().parse() { Ok(x) => x, _ => "".to_string() },
                        "cid" => record.cid = match pair[1].trim().parse() { Ok(x) => x, _ => "".to_string() },
                        _ => {}
                    }
                }
            }
        } else {
            ret.push(record);
            record = Default::default();
        }
    }
    if record != Default::default() {
        ret.push(record);
    }
    ret
}

/***********************************************/

fn part_1(input: &Vec<Record>) {
    let count = input.iter()
        .filter(|r| r.byr != 0 &&
                    r.iyr != 0 &&
                    r.eyr != 0 &&
                   !r.hgt.is_empty() &&
                   !r.hcl.is_empty() &&
                   !r.ecl.is_empty() &&
                   !r.pid.is_empty())
        .collect::<Vec<&Record>>().len();
    println!("Valid records (part 1): {}", count);
}

fn part_2(input: &Vec<Record>) {
    let count = input.iter()
        .filter(|r| match r.byr { 1920 ..= 2002 => true, _ => false } &&
                    match r.iyr { 2010 ..= 2020 => true, _ => false } &&
                    match r.eyr { 2020 ..= 2030 => true, _ => false } &&
                  ((match r.hgt.trim_end_matches("cm").parse::<u16>() { Ok(hgt) => (match hgt { 150 ..= 193 => true, _ => false }), _ => false }) ||
                   (match r.hgt.trim_end_matches("in").parse::<u16>() { Ok(hgt) => (match hgt {  59 ..=  76 => true, _ => false }), _ => false })) &&
                    r.hcl.len() == 7 &&
                    u32::from_str_radix(r.hcl.trim_start_matches('#'), 16).is_ok() &&
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&r.ecl.as_str()) &&
                    r.pid.len() == 9 &&
                    r.pid.parse::<u32>().is_ok())
        .collect::<Vec<&Record>>().len();
    println!("Valid records (part 2): {}", count);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
