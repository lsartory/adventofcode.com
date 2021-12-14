use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211212.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut ret = HashMap::new();
    for line in input {
        let parts: Vec<_> = line.split('-').map(|x| x.trim().to_string()).collect();
        if !ret.contains_key(&parts[0]) {
            ret.insert(parts[0].clone(), Vec::new());
        }
        if let Some(x) = ret.get_mut(&parts[0]) {
            x.push(parts[1].clone());
        }
        if !ret.contains_key(&parts[1]) {
            ret.insert(parts[1].clone(), Vec::new());
        }
        if let Some(x) = ret.get_mut(&parts[1]) {
            x.push(parts[0].clone());
        }
    }
    ret
}

/***********************************************/

fn part_1(input: &HashMap<String, Vec<String>>) {
    let map = input.clone();
    let mut total_paths = 0;
    fn iterate(map: &HashMap<String, Vec<String>>, name: &String, mut visited: Vec<String>, total_paths: &mut u32) {
        visited.push(name.clone());
        if *name == "end".to_string() {
            *total_paths += 1;
            return;
        }
        let cave = &map[name];
        for next in cave {
            if !next.chars().all(|c| c.is_ascii_lowercase()) || !visited.contains(next) {
                iterate(map, next, visited.clone(), total_paths);
            }
        }
    }
    iterate(&map, &"start".to_string(), Vec::new(), &mut total_paths);
    println!("Part 1: {}", total_paths);
}

fn part_2(input: &HashMap<String, Vec<String>>) {
    let map = input.clone();
    let mut total_paths = 0;
    fn iterate(map: &HashMap<String, Vec<String>>, name: &String, mut visited: Vec<String>, total_paths: &mut u32) {
        visited.push(name.clone());
        if *name == "end".to_string() {
            *total_paths += 1;
            return;
        }
        let cave = &map[name];
        for next in cave {
            if *next != "start".to_string() {
                let extra_visit = if visited.iter().filter(|x| x.chars().all(|c| c.is_ascii_lowercase())).map(|x| visited.iter().filter(|y| *y == x).count()).any(|x| x > 1) { 0 } else { 1 };
                if !next.chars().all(|c| c.is_ascii_lowercase()) || visited.iter().filter(|x| *x == next).count() < 1 + extra_visit { 
                    iterate(map, next, visited.clone(), total_paths);
                }
            }
        }
    }
    iterate(&map, &"start".to_string(), Vec::new(), &mut total_paths);
    println!("Part 1: {}", total_paths);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
