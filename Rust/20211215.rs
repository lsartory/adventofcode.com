use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211215.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<Vec<u32>> {
    input.iter().map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(u32::MAX)).collect()).collect()
}

/***********************************************/

fn part_1(input: &Vec<Vec<u32>>) {
    let mut q = Vec::new();
    let mut dist = input.clone();
    let dist_map = dist.as_mut_slice();
    let height = dist_map.len();
    let width = if height > 0 { dist_map[0].len() } else { 0 };
    for y in 0 .. height {
        for x in 0 .. width {
            dist_map[y][x] = u32::MAX;
            q.push((x, y));
        }
    }
    dist_map[0][0] = 0;

    while !q.is_empty() {
        let v = q.iter().enumerate().map(|p| (p.1.0, p.1.1, dist_map[p.1.1][p.1.0], p.0)).min_by(|a, b| a.2.cmp(&b.2)).unwrap_or((0, 0, 0, 0));
        q.swap_remove(v.3);
        for u in q.iter().filter(|a| a.0 == v.0 && ((v.1 > 0 && a.1 == v.1 - 1) || (v.1 < height && a.1 == v.1 + 1)) || a.1 == v.1 && ((v.0 > 0 && a.0 == v.0 - 1) || (v.0 < width && a.0 == v.0 + 1))) {
            dist_map[u.1][u.0] = dist_map[u.1][u.0].min(dist_map[v.1][v.0] + input[u.1][u.0]);
        }
        print!("\r{:5} / {:5}", width * height - q.len(), width * height);
    }
    print!("\r                    ");
    println!("\rPart 1: {}", dist_map[height - 1][width - 1]);
}

fn part_2(input: &Vec<Vec<u32>>) {
    let mut height = input.len();
    let mut width = if height > 0 { input[0].len() } else { 0 };
    let mut new_map = Vec::new();
    for y in 0 .. height * 5 {
        let mut row = Vec::new();
        for x in 0 .. width * 5 {
            row.push(((input[y % height][x % width] - 1 + (x / width) as u32 + (y / height) as u32) % 9) + 1);
        }
        new_map.push(row);
    }
    width *= 5;
    height *= 5;

    let mut q = Vec::new();
    let mut dist = new_map.clone();
    let dist_map = dist.as_mut_slice();
    for y in 0 .. height {
        for x in 0 .. width {
            dist_map[y][x] = u32::MAX;
            q.push((x, y));
        }
    }
    dist_map[0][0] = 0;

    while !q.is_empty() {
        let v = q.iter().enumerate().map(|p| (p.1.0, p.1.1, dist_map[p.1.1][p.1.0], p.0)).min_by(|a, b| a.2.cmp(&b.2)).unwrap_or((0, 0, 0, 0));
        q.swap_remove(v.3);
        for u in q.iter().filter(|a| a.0 == v.0 && ((v.1 > 0 && a.1 == v.1 - 1) || (v.1 < height && a.1 == v.1 + 1)) || a.1 == v.1 && ((v.0 > 0 && a.0 == v.0 - 1) || (v.0 < width && a.0 == v.0 + 1))) {
            dist_map[u.1][u.0] = dist_map[u.1][u.0].min(dist_map[v.1][v.0] + new_map[u.1][u.0]);
        }
        print!("\r{:6} / {:6}", width * height - q.len(), width * height);
    }
    print!("\r                    ");
    println!("\rPart 2: {}", dist_map[height - 1][width - 1]);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
