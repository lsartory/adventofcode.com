use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201206.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

struct Group {
    size:    usize,
    answers: Vec<u32>
}
impl Default for Group {
    fn default() -> Group {
        Group {
            size: 0,
            answers: vec![0; 26]
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Group> {
    let mut ret = Vec::new();
    let mut group:Group = Default::default();
    for line in input {
        if !line.is_empty() {
            for c in line.chars() {
                match c {
                    'a' ..= 'z' => group.answers[c as usize - 'a' as usize] += 1,
                    _ => {}
                }
            }
            group.size += 1;
        } else if group.size != 0 {
            ret.push(group);
            group = Default::default();
        }
    }
    if group.size != 0 {
        ret.push(group);
    }
    ret
}

/***********************************************/

fn part_1(input: &Vec<Group>) {
    println!("Part 1: {}", input.iter().map(|g| g.answers.iter().map(|a| if *a != 0 { 1 } else { 0 }).sum::<u32>()).collect::<Vec<u32>>().iter().sum::<u32>());
}

fn part_2(input: &Vec<Group>) {
    println!("Part 2: {}", input.iter().map(|g| g.answers.iter().map(|a| a / g.size as u32).sum::<u32>()).collect::<Vec<u32>>().iter().sum::<u32>());
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
