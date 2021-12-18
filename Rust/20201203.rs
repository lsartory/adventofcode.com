use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201203.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<bool>> {
    input.into_iter().map(|line| line.chars().map(|c| c == '#').collect()).collect()
}

/***********************************************/

fn part_1(input: &Vec<Vec<bool>>) {
    let mut tree_count = 0;
    for (x, row) in input.iter().enumerate() {
        print!("{} → ", x);
        for (y, col) in row.iter().enumerate() {
            if y == (x * 3) % row.len() {
                print!("{}", if *col { tree_count += 1; 'X' } else { 'O' });
            } else {
                print!("{}", if *col { '#' } else { '.' });
            }
        }
        println!();
    }
    println!("Tree count: {}", tree_count);
}

fn part_2(input: &Vec<Vec<bool>>) {
    let mut tree_product: u64 = 1;
    for pair in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let mut tree_count = 0;
        for i in 0..(input.len() / pair.1) {
            if input[i * pair.1][(i * pair.0) % input[i].len()] {
                tree_count += 1;
            }
        }
        tree_product *= tree_count;
    }
    println!("Tree product: {}", tree_product);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
