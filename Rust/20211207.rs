use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211207.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<u32> {
    (match input.first() { Some(x) => x, _ => "" }).split(',').map(|x| match x.trim().parse::<u32>() { Ok(x) => x, _ => 0 }).collect()
}

/***********************************************/

fn part_1(input: &Vec<u32>) {
    let mut costs = Vec::new();
    let range = (match input.iter().min() { Some(x) => *x, _ => 0 }, match input.iter().max() { Some(x) => *x, _ => 0 });
    for i in range.0..=range.1 {
        costs.push(input.iter().fold(0, |accum, x| accum + if i > *x { i - x } else { x - i }));
    }
    let best = match costs.iter().enumerate().reduce(|accum, x| if accum.1 < x.1 { accum } else { x }) { Some(x) => x, _ => (0, &0) };
    println!("Part 1: Best position is {}, with {} fuel required.", best.0, best.1);
}

fn part_2(input: &Vec<u32>) {
    let mut costs = Vec::new();
    let range = (match input.iter().min() { Some(x) => *x, _ => 0 }, match input.iter().max() { Some(x) => *x, _ => 0 });
    for i in range.0..=range.1 {
        costs.push(input.iter().fold(0, |accum, x| accum + { let n = if i > *x { i - x } else { x - i }; (n * (n + 1)) / 2 }));
    }
    let best = match costs.iter().enumerate().reduce(|accum, x| if accum.1 < x.1 { accum } else { x }) { Some(x) => x, _ => (0, &0) };
    println!("Part 2: Best position is {}, with {} fuel required.", best.0, best.1);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
