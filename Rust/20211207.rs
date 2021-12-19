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
    (input.first().unwrap_or(&String::new())).split(',').filter_map(|x| x.trim().parse::<u32>().ok()).collect()
}

/***********************************************/

fn part_1(input: &[u32]) {
    let mut costs = Vec::new();
    for i in *input.iter().min().unwrap_or(&0) ..= *input.iter().max().unwrap_or(&0) {
        costs.push(input.iter().fold(0, |accum, x| accum + if i > *x { i - x } else { x - i }));
    }
    let best = costs.iter().enumerate().reduce(|accum, x| if accum.1 < x.1 { accum } else { x }).unwrap_or((0, &0));
    println!("Part 1: Best position is {}, with {} fuel required.", best.0, best.1);
}

fn part_2(input: &[u32]) {
    let mut costs = Vec::new();
    for i in *input.iter().min().unwrap_or(&0) ..= *input.iter().max().unwrap_or(&0) {
        costs.push(input.iter().fold(0, |accum, x| accum + { let n = if i > *x { i - x } else { x - i }; (n * (n + 1)) / 2 }));
    }
    let best = costs.iter().enumerate().reduce(|accum, x| if accum.1 < x.1 { accum } else { x }).unwrap_or((0, &0));
    println!("Part 2: Best position is {}, with {} fuel required.", best.0, best.1);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
