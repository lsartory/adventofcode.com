use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201201.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<i32>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn part_1(input: &Vec<i32>) {
    for (i, x) in input.iter().enumerate() {
        for y in &input[i + 1 ..] {
            if x + y == 2020 {
                println!("{} × {} = {}", x, y, x * y);
                return;
            }
        }
    }
}

fn part_2(input: &Vec<i32>) {
    for (i, x) in input.iter().enumerate() {
        for (j, y) in input[i + 1 ..].iter().enumerate() {
            for z in &input[j + 1 ..] {
                if x + y + z == 2020 {
                    println!("{} × {} × {} = {}", x, y, z, x * y * z);
                    return;
                }
            }
        }
    }
}

/***********************************************/

fn main() {
    let input = read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE));
    part_1(&input);
    part_2(&input);
}
