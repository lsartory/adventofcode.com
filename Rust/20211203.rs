use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211203.txt";
const BIT_WIDTH:usize = 12;

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<u32>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| u32::from_str_radix(line?.trim(), 2).map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn part_1(input: &Vec<u32>) {
    let mut gamma = 0;
    for i in (0..BIT_WIDTH).rev() {
        gamma <<= 1;
        gamma |= input.iter().map(|x| (x >> i) & 1).sum::<u32>() / (input.len() as u32 >> 1);
    }
    let epsilon = (!gamma) & ((1 << BIT_WIDTH) - 1);
    println!("Gamma = {}, epsilon = {}, Product = {}", gamma, epsilon, gamma * epsilon);
}

fn part_2(input: &Vec<u32>) {
    let mut list = input.clone();
    for i in (0..BIT_WIDTH).rev() {
        let count = list.iter().map(|x| (x >> i) & 1).sum::<u32>();
        let val = if count >= list.len() as u32 - count { 1 } else { 0 };
        list = list.iter().filter(|x| (**x >> i) & 1 == val).cloned().collect();
        if list.len() == 1 {
            break;
        }
    }
    let o2 = match list.first() { Some(x) => *x, _ => 0 };
    list = input.clone();
    for i in (0..BIT_WIDTH).rev() {
        let count = list.iter().map(|x| (x >> i) & 1).sum::<u32>();
        let val = if count < list.len() as u32 - count { 1 } else { 0 };
        list = list.iter().filter(|x| (**x >> i) & 1 == val).cloned().collect();
        if list.len() == 1 {
            break;
        }
    }
    let co2 = match list.first() { Some(x) => *x, _ => 0 };
    println!("O2 rating = {}, CO2 rating = {}, Product = {}", o2, co2, o2 * co2);
}

/***********************************************/

fn main() {
    let input = read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE));
    part_1(&input);
    part_2(&input);
}
