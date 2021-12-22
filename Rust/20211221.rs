use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211221.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> (u32, u32) {
    let positions = input.iter().map(|line| line.split(' ').filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<_>>()).collect::<Vec<_>>().concat();
    if positions.len() >= 4 { (positions[1], positions[3]) } else { (0, 0) }
}

/***********************************************/

fn part_1(input: &(u32, u32)) {
    let mut scores = [0, 0];
    let mut positions = [input.0, input.1];
    let mut next_roll = 6;
    let mut roll_count = 0;
    'outer: loop {
        for i in 0 .. 2 {
            positions[i] = (positions[i] + next_roll - 1) % 10 + 1;
            scores[i] += positions[i];
            roll_count += 3;
            if scores[i] >= 1000 {
                break 'outer;
            }
            next_roll = (next_roll % 100) + 9;
        }
    }
    println!("Part 1: {}", scores[0].min(scores[1]) * roll_count);
}

fn part_2(input: &(u32, u32)) {
    let mut frequencies = vec![0; 3 * 3 + 1];
    for i in 1 ..= 3 {
        for j in 1 ..= 3 {
            for k in 1 ..= 3 {
                frequencies[i + j + k] += 1;
            }
        }
    }
    type Cache = HashMap<((u32, u32), (u32, u32), usize), (u64, u64)>;
    fn iterate(frequencies: &[u32], cache: &mut Cache, turn: usize, scores: &[u32], positions: &[u32]) -> (u64, u64) {
        let mut total_wins = (0, 0);
        for (i, score) in scores.iter().enumerate() {
            if *score >= 21 {
                return if i == 0 { (1, 0) } else { (0, 1) };
            }
        }
        let player = (turn & 1) as usize;
        for (roll, freq) in frequencies.iter().enumerate().filter(|x| *x.1 != 0) {
            let mut new_positions = positions.to_owned();
            let mut new_scores = scores.to_owned();
            new_positions[player] = (new_positions[player] + roll as u32 - 1) % 10 + 1;
            new_scores[player] += new_positions[player];
            let cache_key = ((new_scores[0], new_scores[1]), (new_positions[0], new_positions[1]), player ^ 1);
            if !cache.contains_key(&cache_key) {
                let ret = iterate(frequencies, cache, turn + 1, &new_scores, &new_positions);
                cache.insert(cache_key, ret);
            }
            let results = cache.get(&cache_key).unwrap();
            total_wins.0 += results.0 * *freq as u64;
            total_wins.1 += results.1 * *freq as u64;
        }
        total_wins
    }
    let wins = iterate(&frequencies, &mut HashMap::new(), 0, &[0, 0], &[input.0, input.1]);
    println!("Part 2: {:?}", wins.0.max(wins.1));
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
