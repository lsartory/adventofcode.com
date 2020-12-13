use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201209.txt";
const PREAMBLE_LENGTH: usize = 25;

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<u64>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn invalidate(set: &[u64], value: &u64) -> Option<u64> {
    for (i, x) in set.iter().enumerate() {
        for y in &set[i + 1 ..] {
            if x + y == *value {
                return None;
            }
        }
    }
    Some(*value)
}

fn part_1(input: &Vec<u64>) -> Result<u64> {
    for i in input.iter().enumerate().filter_map(|(i, x)| if i < PREAMBLE_LENGTH { None } else { invalidate(&input[i - PREAMBLE_LENGTH .. i], x) }) {
        println!("Found invalid value: {}", i);
        return Ok(i);
    }
    Err(Error::new(ErrorKind::NotFound, "Invalid value not found"))
}

fn part_2(input: &Vec<u64>, invalid_value: u64) {
    let mut start = 0;
    let mut end   = 0;
    'outer: for (i, x) in input.iter().enumerate() {
        let mut sum = *x;
        for (j, y) in input[i + 1 ..].iter().enumerate() {
            sum += y;
            if sum > invalid_value {
                continue;
            } else if sum == invalid_value {
                start = i;
                end   = i + j + 1;
                break 'outer;
            }
        }
    }
    if start != 0 && end > start {
        let slice = &input[start ..= end];
        println!("Found matching range from {} to {}", start, end);
        println!("Product of min and max: {}", slice.iter().min().unwrap() + slice.iter().max().unwrap());
    } else {
        println!("Found no matching range.");
    }
}

/***********************************************/

fn main() {
    let input = read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE));
    println!("Total records: {}", input.len());
    let invalid_value = part_1(&input).expect("Cannot proceed");
    part_2(&input, invalid_value);
}
