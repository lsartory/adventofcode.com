use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201216.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> (HashMap<String, Vec<(u16, u16)>>, Vec<Vec<u16>>) {
    let mut rules   = HashMap::new();
    let mut tickets = Vec::new();
    let mut section = 0;

    for line in input {
        if line.is_empty() {
            continue;
        }
        match line.as_str() {
            "your ticket:"    => section = 1,
            "nearby tickets:" => section = 2,
            _ => match section {
                0 => {
                    let mut key   = "";
                    let mut range = (0, 0);
                    let mut ranges = Vec::new();

                    for (i, part) in line.split(':').map(|x| x.trim()).enumerate() {
                        match i {
                            0 => key = part,
                            1 => for (i, value) in part.replace("or", "-").split("-").filter_map(|x| x.trim().parse::<u16>().ok()).enumerate() {
                                if i & 1 == 0 {
                                    range.0 = value;
                                } else {
                                    range.1 = value;
                                    ranges.push(range);
                                }
                            }
                            _ => continue
                        }
                    }
                    if !key.is_empty() && !ranges.is_empty() {
                        rules.insert(key.to_string(), ranges);
                    }
                },
                1 | 2 => {
                    tickets.push(line.split(',').filter_map(|x| x.trim().parse::<u16>().ok()).collect());
                },
                _ => {}
            }
        }
    }
    (rules, tickets)
}

/***********************************************/

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
{
    (0 .. v[0].len()).map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>()).collect()
}

fn part_1(input: &(HashMap<String, Vec<(u16, u16)>>, Vec<Vec<u16>>)) {
    let ranges: Vec<(u16, u16)> = input.0.values().cloned().collect::<Vec<Vec<(u16, u16)>>>().iter().flatten().cloned().collect();
    let error_rate = input.1.iter().map(|ticket| ticket.iter().filter(|field| ranges.iter().map(|x| if **field >= x.0 && **field <= x.1 { 1 } else { 0 }).sum::<u64>() == 0).map(|field| u64::from(*field)).sum::<u64>()).sum::<u64>();
    println!("Ticket scanning error rate: {}", error_rate);
}

fn part_2(input: &(HashMap<String, Vec<(u16, u16)>>, Vec<Vec<u16>>)) {
    let ranges: Vec<(u16, u16)> = input.0.values().cloned().collect::<Vec<Vec<(u16, u16)>>>().iter().flatten().cloned().collect();
    let valid_tickets: Vec<Vec<u16>> = input.1.iter().skip(1).filter(|ticket| ticket.iter().filter(|field| ranges.iter().map(|x| if **field >= x.0 && **field <= x.1 { 1 } else { 0 }).sum::<u64>() == 0).map(|field| u64::from(*field)).sum::<u64>() == 0).cloned().collect();

    let mut names = Vec::new();
    for col in transpose(valid_tickets) {
        let mut name = Vec::new();
        'rule_loop: for rule in &input.0 {
            for value in &col {
                let mut ok = false;
                for range in rule.1 {
                    if *value >= range.0 && *value <= range.1 {
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    continue 'rule_loop;
                }
            }
            name.push(rule.0);
        }
        names.push(name);
    }

    // TODO: check the names filtering
    // TODO: remove extra names

    for (i, name) in names.iter().enumerate() {
        println!("{} → {} - {:?}", i, name.len(), name);
    }
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.0.len() + input.1.len());
    part_1(&input);
    part_2(&input);
}
