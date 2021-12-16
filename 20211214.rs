use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211214.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> (String, Vec<(String, String)>) {
    let mut template = String::new();
    let mut rules = Vec::new();
    for (i, line) in input.iter().enumerate() {
        if i == 0 {
            template = line.clone();
        } else if i > 1 {
            let pair = line.split("->").map(|x| x.trim().to_string()).collect::<Vec<_>>();
            if pair.len() > 1 {
                rules.push((pair[0].clone(), pair[1].clone()));
            }
        }
    }
    (template, rules)
}

/***********************************************/

fn part_1(input: &(String, Vec<(String, String)>)) {
    let mut base = input.0.clone();
    let mut polymer = String::new();
    for _i in 0 .. 10 {
        for i in base.chars().collect::<Vec<char>>().windows(2) {
            polymer.push(i[0]);
            let seq = i.iter().collect::<String>();
            if let Some(rule) = input.1.iter().find(|x| x.0 == seq) {
                polymer += &rule.1;
            }
        }
        polymer.push(input.0.chars().last().unwrap_or('#'));
        base = polymer.clone();
        polymer.clear();
    }
    let mut elements: Vec<_> = base.clone().chars().collect();
    elements.sort_unstable();
    elements.dedup();
    let occurences = elements.iter().map(|x| base.chars().filter(|y| y == x).count()).collect::<Vec<usize>>();
    println!("Part 1: {}", occurences.iter().max().unwrap_or(&0) - occurences.iter().min().unwrap_or(&0));
}

fn part_2(input: &(String, Vec<(String, String)>)) {
    let mut pairs = HashMap::new();
    for p in &input.1 {
        pairs.insert(p.0.clone(), 0u64);
    }
    for i in input.0.chars().collect::<Vec<_>>().windows(2) {
        *pairs.entry(i.iter().collect()).or_insert(0) += 1;
    }

    for _ in 0 .. 40 {
        let tmp_pairs = pairs.clone();
        pairs.clear();
        for p in &input.1 {
            let count = tmp_pairs.get(&p.0).unwrap_or(&0);
            *pairs.entry(p.0[0..1].to_string() + &p.1).or_insert(0) += count;
            *pairs.entry(p.1.to_string() + &p.0[1..2]).or_insert(0) += count;
        }
    }

    let mut occurences = HashMap::new();
    for p in pairs.iter().filter(|x| *x.1 > 0) {
        *occurences.entry(p.0[0..1].to_string()).or_insert(0) += p.1;
    }
    *occurences.entry(input.0[input.0.len() - 1 .. input.0.len()].to_string()).or_insert(0) += 1;
    println!("Part 2: {}", occurences.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap_or((&"".to_string(), &0)).1 - occurences.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap_or((&"".to_string(), &0)).1);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
