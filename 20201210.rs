use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201210.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<u64>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: &Vec<u64>) -> Vec<u64> {
    let mut list = input.clone();
    list.sort_unstable();
    list.push(match list.last() { Some(x) => *x, _ => 0 } + 3);
    list
}

/***********************************************/

fn part_1(input: &Vec<u64>) {
    let mut prev = 0;
    let delta = input.iter().map(|x| { let prev_tmp = prev; prev = *x; x - prev_tmp }).collect::<Vec<u64>>();
    let delta_1 = delta.iter().filter(|x| **x == 1).count();
    let delta_3 = delta.iter().filter(|x| **x == 3).count();
    println!("1-jolt delta count: {}; 3-jolt delta count: {}; Jolt-product: {}", delta_1, delta_3, delta_1 * delta_3);
}

fn part_2(input: &Vec<u64>) {
    let mut combinations: u64 = 1;
    let mut prev = 0;
    let delta = input.iter().map(|x| { let prev_tmp = prev; prev = *x; x - prev_tmp }).collect::<Vec<u64>>();
    let mut streak = 0;
    for d in delta {
        if d == 1 {
            streak += 1;
        } else if streak != 0 {
            combinations *= (1 << (streak - 1)) - if streak <= 3 { 0 } else { (streak << 1) - 7 };
            streak = 0;
        }
    }
    println!("Total possible combinations: {}", combinations);
}

/***********************************************/

fn main() {
    let input = parse_input(&read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
