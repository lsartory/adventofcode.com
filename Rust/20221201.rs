use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
 
/***********************************************/
 
const INPUT_FILE:&str = "20221201.txt";
 
/***********************************************/
 
fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}
 
/***********************************************/
 
fn parse_input(input: Vec<String>) -> Vec<Vec<u32>> {
    let mut ret = Vec::new();
    let mut group = Vec::new();
    for line in input {
        if let Ok(x) = line.parse::<u32>() {
            group.push(x);
        } else {
            ret.push(group);
            group = Vec::new();
        }
    }
    ret.push(group);
    ret
}
 
/***********************************************/
 
fn part_1(input: &[Vec<u32>]) {
    println!("Part 1: {}", match input.iter().map(|x| x.iter().sum::<u32>()).max() { Some(x) => x.to_string(), _ => "<Error>".to_string() });
}
 
fn part_2(input: &[Vec<u32>]) {
    let mut sums = input.iter().map(|x| x.iter().sum()).collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a));
    println!("Part 2: {}", &sums[0..=2].iter().sum::<u32>());
}
 
/***********************************************/
 
fn main() {
    let input = parse_input(read_input(INPUT_FILE).unwrap_or_else(|_| panic!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
