use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211205.txt";
const GRID_ROWS:usize = 1000;
const GRID_COLS:usize = 1000;

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<((u32, u32), (u32, u32))> {
    let mut ret = Vec::new();
    for line in input {
        let parts: Vec<u32> = line.split("->").map(|x| x.trim().split(',').map(|x| match x.trim().parse::<u32>() { Ok(x) => x, _ => 0 }).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>().into_iter().flat_map(|x| x.into_iter()).collect();
        ret.push(((parts[0], parts[1]), (parts[2], parts[3])));
    }
    ret
}

/***********************************************/

fn part_1(input: &Vec<((u32, u32), (u32, u32))>) {
    let mut grid = [[0; GRID_ROWS]; GRID_COLS];
    for line in input {
        if line.0.0 == line.1.0 || line.0.1 == line.1.1 {
            let x_range = (if line.0.0 < line.1.0 { line.0.0 } else { line.1.0 }, if line.0.0 > line.1.0 { line.0.0 } else { line.1.0 });
            let y_range = (if line.0.1 < line.1.1 { line.0.1 } else { line.1.1 }, if line.0.1 > line.1.1 { line.0.1 } else { line.1.1 });
            for x in x_range.0..=x_range.1 {
                for y in y_range.0..=y_range.1 {
                    grid[y as usize][x as usize] += 1;
                }
            }
        }
    }
    let sum = grid.iter().flat_map(|x| x.iter()).filter(|x| **x > 1).count();
    println!("Part 1: {}", sum);
}

fn part_2(input: &Vec<((u32, u32), (u32, u32))>) {
    let mut grid = [[0; GRID_ROWS]; GRID_COLS];
    for line in input {
        let mut x = line.0.0;
        let mut y = line.0.1;
        loop {
            grid[y as usize][x as usize] += 1;
            if x == line.1.0 && y == line.1.1 {
                break;
            }
            if x != line.1.0 {
                x = if line.0.0 < line.1.0 { x + 1 } else { x - 1 };
            }
            if y != line.1.1 {
                y = if line.0.1 < line.1.1 { y + 1 } else { y - 1 };
            }
        }
    }
    let sum = grid.iter().flat_map(|x| x.iter()).filter(|x| **x > 1).count();
    println!("Part 2: {}", sum);
    /*
    for row in grid {
        for col in row {
            print!("{}", if col != 0 { col.to_string() } else { ".".to_string() });
        }
        println!();
    }
    */
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
