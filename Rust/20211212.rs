use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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

fn get_hash(x: String) -> u64 {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

fn parse_input(input: Vec<String>) -> HashMap<u64, Vec<(u64, bool)>> {
    let mut ret = HashMap::new();
    for line in input {
        let parts: Vec<_> = line.split('-').map(|x| x.trim().to_string()).collect();
        let hashes = (get_hash(parts[0].clone()), get_hash(parts[1].clone()));
        let types = (parts[0].chars().all(|c| c.is_ascii_lowercase()), parts[1].chars().all(|c| c.is_ascii_lowercase()));
        if !ret.contains_key(&hashes.0) {
            ret.insert(hashes.0, Vec::new());
        }
        if let Some(x) = ret.get_mut(&hashes.0) {
            x.push((hashes.1, types.1));
        }
        if !ret.contains_key(&hashes.1) {
            ret.insert(hashes.1, Vec::new());
        }
        if let Some(x) = ret.get_mut(&hashes.1) {
            x.push((hashes.0, types.0));
        }
    }
    ret
}

/***********************************************/

fn part_1(input: &HashMap<u64, Vec<(u64, bool)>>) {
    let map = input.clone();
    let mut total_paths = 0;
    fn iterate(map: &HashMap<u64, Vec<(u64, bool)>>, cave: u64, end_cave: u64, mut visited: Vec<u64>, total_paths: &mut u32) {
        visited.push(cave);
        if cave == end_cave {
            *total_paths += 1;
            return;
        }
        for next in &map[&cave] {
            if !next.1 || !visited.contains(&next.0) {
                iterate(map, next.0, end_cave, visited.clone(), total_paths);
            }
        }
    }
    iterate(&map, get_hash("start".to_string()), get_hash("end".to_string()), Vec::new(), &mut total_paths);
    println!("Part 1: {}", total_paths);
}

fn part_2(input: &HashMap<u64, Vec<(u64, bool)>>) {
    let map = input.clone();
    let mut total_paths = 0;
    fn iterate(map: &HashMap<u64, Vec<(u64, bool)>>, cave: (u64, bool), start_cave: u64, end_cave: u64, mut visited: HashMap<u64, u32>, extra_allowed: bool, total_paths: &mut u32) {
        if cave.0 == end_cave {
            *total_paths += 1;
            return;
        }
        if cave.1 {
            *visited.entry(cave.0).or_insert(0) += 1;
        }
        let extra_visit = if !extra_allowed || match visited.iter().map(|x| *x.1).max() { Some(x) => x, _ => 0 } > 1 { 0 } else { 1 };
        for next in &map[&cave.0] {
            if next.0 != start_cave && (!next.1 || *visited.entry(next.0).or_insert(0) < 1 + extra_visit) {
                iterate(map, *next, start_cave, end_cave, visited.clone(), extra_visit != 0, total_paths);
            }
        }
    }
    iterate(&map, (get_hash("start".to_string()), true), get_hash("start".to_string()), get_hash("end".to_string()), HashMap::new(), true, &mut total_paths);
    println!("Part 2: {}", total_paths);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
