use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211213.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> (Vec<(u32, u32)>, Vec<(char, u32)>) {
    let mut points = Vec::new();
    let mut folds = Vec::new();
    for line in input {
        let coords = line.split(',').filter_map(|x| x.trim().parse::<u32>().ok()).collect::<Vec<u32>>();
        if coords.len() > 1 {
            points.push((coords[0], coords[1]));
        }
        let instructions = line.split('=').map(|x| x.trim().to_string()).collect::<Vec<String>>();
        if instructions.len() > 1 {
            folds.push((instructions[0].chars().last().unwrap_or(' '), instructions[1].parse().unwrap_or(0)));
        }
    }
    (points, folds)
}

/***********************************************/

fn part_1(input: &(Vec<(u32, u32)>, Vec<(char, u32)>)) {
    let mut grid = input.0.clone();
    let fold = input.1.first().unwrap_or(&(' ', 0));
    for i in grid.as_mut_slice() {
        if fold.0 == 'x' && i.0 > fold.1 {
            i.0 = fold.1 - (i.0 - fold.1);
        } else if fold.0 == 'y' && i.1 > fold.1 {
            i.1 = fold.1 - (i.1 - fold.1);
        }
    }
    grid.sort_unstable();
    grid.dedup();
    println!("Part 1: {}", grid.len());
}

fn part_2(input: &(Vec<(u32, u32)>, Vec<(char, u32)>)) {
    let mut grid = input.0.clone();
    for fold in &input.1 {
        for i in grid.as_mut_slice() {
            if fold.0 == 'x' && i.0 > fold.1 {
                i.0 = fold.1 - (i.0 - fold.1);
            } else if fold.0 == 'y' && i.1 > fold.1 {
                i.1 = fold.1 - (i.1 - fold.1);
            }
        }
    }
    grid.sort_unstable();
    grid.dedup();
    println!("Part 2:");
    for y in grid.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap_or(&(0, 0)).1 ..= grid.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap_or(&(0, 20)).1 {
        for x in grid.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap_or(&(0, 0)).0 ..= grid.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap_or(&(100, 0)).0 {
            print!("{}", if grid.contains(&(x, y)) { '#' } else { ' ' });
        }
        println!();
    }
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
