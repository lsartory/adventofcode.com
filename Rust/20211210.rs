use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211210.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn part_1(input: &[String]) {
    let mut score = 0;
    for line in input {
        let mut stack = Vec::new();
        //print!("{}", line);
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => { stack.push(c); },
                                    _ => {
                    let points = |c| match c {
                        ')' =>     3,
                        ']' =>    57,
                        '}' =>  1197,
                        '>' => 25137,
                          _ =>     0
                    };
                    match stack.pop() {
                        Some('(') => { if c != ')' { /* print!(" - Expected ')', but found '{}' instead.",  c); */ score += points(c); break; }},
                        Some('[') => { if c != ']' { /* print!(" - Expected ']', but found '{}' instead.",  c); */ score += points(c); break; }},
                        Some('{') => { if c != '}' { /* print!(" - Expected '}}', but found '{}' instead.", c); */ score += points(c); break; }},
                        Some('<') => { if c != '>' { /* print!(" - Expected '>', but found '{}' instead.",  c); */ score += points(c); break; }},
                                _ => {}
                    }
                }
            }
        }
        //println!();
    }
    println!("Part 1: {}", score);
}

fn part_2(input: &[String]) {
    let mut scores = Vec::new();
    'outer: for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => { stack.push(c); },
                                    _ => {
                    match stack.pop() {
                        Some('(') => { if c != ')' { continue 'outer; }},
                        Some('[') => { if c != ']' { continue 'outer; }},
                        Some('{') => { if c != '}' { continue 'outer; }},
                        Some('<') => { if c != '>' { continue 'outer; }},
                                _ => {}
                    }
                }
            }
        }
        scores.push(stack.iter().rev().fold(0, |score: u64, c| (score * 5) + match c { '(' => 1, '[' => 2, '{' => 3, '<' => 4, _ => 0 }));
    }
    scores.sort_unstable();
    println!("Part 2: {}", scores[scores.len() / 2]);
}

/***********************************************/

fn main() {
    let input = read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE));
    part_1(&input);
    part_2(&input);
}
