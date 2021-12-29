use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211225.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(Clone, PartialEq)]
enum Cell { East, South, Empty }

fn parse_input(input: Vec<String>) -> Vec<Vec<Cell>> {
    input.iter().map(|x| x.chars().filter_map(|c| match c { '.' => Some(Cell::Empty), '>' => Some(Cell::East), 'v' => Some(Cell::South), _ => None }).collect::<Vec<_>>()).collect::<Vec<_>>()
}

/***********************************************/

fn part_1(input: &[Vec<Cell>]) {
    let mut map = input.to_owned();
    let mut new_map = map.clone();
    let height = map.len();
    let width = if height != 0 { map[0].len() } else { 0 };

    let mut i = 1;
    loop {
        let mut moved = false;

        for y in 0 .. height {
            for x in 0 .. width {
                if map[y][x] == Cell::East && map[y][(x + 1) % width] == Cell::Empty {
                    moved = true;
                    new_map[y][x] = Cell::Empty;
                    new_map[y][(x + 1) % width] = Cell::East;
                }
            }
        }
        map = new_map.clone();

        for y in 0 .. height {
            for x in 0 .. width {
                if map[y][x] == Cell::South && map[(y + 1) % height][x] == Cell::Empty {
                    moved = true;
                    new_map[y][x] = Cell::Empty;
                    new_map[(y + 1) % height][x] = Cell::South;
                }
            }
        }
        map = new_map.clone();

        if !moved {
            break;
        }
        i += 1;
    }

    for y in 0 .. height {
        for x in 0 .. width {
            print!("{}", match map[y][x] { Cell::East => '>', Cell::South => 'v', _ => '.' });
        }
        println!();
    }
    println!();

    println!("Part 1: {}", i);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
}
