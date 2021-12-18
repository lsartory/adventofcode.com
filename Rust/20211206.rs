use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211206.txt";

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
    let duration = 80;
    let mut fishes = input.clone();
    for _i in 0..duration {
        let mut new = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 7;
                new += 1;
            }
        }
        if new > 0 {
            fishes.resize_with(fishes.len() + new, || 9);
        }
        fishes = fishes.iter().map(|x| x - 1).collect();
    }
    println!("Part 1: There are {} fishes after {} days", fishes.len(), duration);
}

fn part_2(input: &Vec<u32>) {
    let duration = 256;
    let mut days = vec![0; 10];
    for i in input {
        days[*i as usize] += 1;
    }
    for _i in 0..duration {
        days[7] += days[0];
        days[9] += days[0];
        for j in 1..days.len() {
            days[j - 1] = days[j];
        }
        days[9] = 0;
    }
    println!("Part 2: There are {} fishes after {} days", days.iter().sum::<u64>(), duration);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
