use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211201.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<i32>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn part_1(input: &[i32]) {
    println!("Part 1: {}", input.windows(2).filter(|x| x[0] < x[1]).count());
}

fn part_2(input: &[i32]) {
    //println!("Part 2: {}", input.windows(3).map(|x| x.iter().sum::<i32>()).collect::<Vec<i32>>().windows(2).filter(|x| x[0] < x[1]).count());
    println!("Part 2: {}", input.windows(4).filter(|x| x[0] < x[3]).count());
}

/***********************************************/

fn main() {
    let input = read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE));
    part_1(&input);
    part_2(&input);
}
