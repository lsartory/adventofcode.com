use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201214.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn part_1(input: &Vec<String>) {
    let mut mem: BTreeMap<u64, u64> = BTreeMap::new();
    let mut mask: (u64, u64) = (u64::MAX, 0);
    for line in input {
        match line.replace(' ', "").strip_prefix("mask=") {
            Some(x) => {
                mask = (match u64::from_str_radix(x.replace(&['0', '1'][..], "0").replace('X', "1").as_str(), 2) { Ok(x) => x, _ => continue },
                        match u64::from_str_radix(x.replace('X', "0").as_str(), 2) { Ok(x) => x, _ => continue });
                continue;
            },
            _ => {}
        }
        match line.replace(&[' ', ']'][..], "").strip_prefix("mem[") {
            Some(x) => {
                let pair: Vec<&str> = x.split('=').collect();
                if pair.len() == 2 {
                    let lhs = match pair[0].parse::<u64>() { Ok(x) => x, _ => continue };
                    let rhs = match pair[1].parse::<u64>() { Ok(x) => x, _ => continue };
                    mem.insert(lhs, (rhs & mask.0) | mask.1);
                }
                continue;
            },
            _ => {}
        }
    }
    println!("Memory sum (part 1): {}", mem.iter().map(|x| x.1).sum::<u64>());
}

fn part_2(input: &Vec<String>) {
    let mut mem: BTreeMap<u64, u64> = BTreeMap::new();
    let mut mask: (u64, u64) = (u64::MAX, 0);
    for line in input {
        match line.replace(' ', "").strip_prefix("mask=") {
            Some(x) => {
                mask = (match u64::from_str_radix(x.replace(&['0', '1'][..], "0").replace('X', "1").as_str(), 2) { Ok(x) => x, _ => continue },
                        match u64::from_str_radix(x.replace('X', "0").as_str(), 2) { Ok(x) => x, _ => continue });
                continue;
            },
            _ => {}
        }
        match line.replace(&[' ', ']'][..], "").strip_prefix("mem[") {
            Some(x) => {
                let pair: Vec<&str> = x.split('=').collect();
                if pair.len() == 2 {
                    let lhs = match pair[0].parse::<u64>() { Ok(x) => x, _ => continue };
                    let rhs = match pair[1].parse::<u64>() { Ok(x) => x, _ => continue };
                    for i in 0 .. (1 << mask.0.count_ones()) {
                        let mut addr: u64 = 0;
                        let mut tmp_mask = mask.0;
                        let mut tmp_lhs = lhs | mask.1;
                        let mut tmp_i = i;
                        for j in 0 .. 64 {
                            if (tmp_mask & 1) != 0 {
                                addr |= (tmp_i & 1) << j;
                                tmp_i >>= 1;
                            } else {
                                addr |= (tmp_lhs & 1) << j;
                            }
                            tmp_mask = tmp_mask >> 1;
                            tmp_lhs >>= 1;
                            if tmp_mask == 0 && tmp_lhs == 0 {
                                break;
                            }
                        }
                        mem.insert(addr, rhs);
                    }
                }
                continue;
            },
            _ => {}
        }
    }
    println!("Memory sum (part 2): {}", mem.iter().map(|x| x.1).sum::<u64>());

}

/***********************************************/

fn main() {
    let input = read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
