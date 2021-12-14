use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211211.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<Vec<Option<u32>>> {
    let mut map = Vec::new();
    for line in input {
        let mut row: Vec<_> = line.split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>().iter().map(|x| match x.trim().parse() { Ok(x) => Some(x), _ => None }).collect();
        row.insert(0, None);
        row.push(None);
        map.push(row);
    }
    if map.len() > 0 {
        let width = map[0].len();
        map.insert(0, vec![None; width]);
        map.push(vec![None; width]);
    }
    map
}

/***********************************************/

fn part_1(input: &Vec<Vec<Option<u32>>>) {
    let mut map_vec = input.clone();
    let height = map_vec.len();
    let width = if height != 0 { map_vec[0].len() } else { 0 };
    let map = map_vec.as_mut_slice();
    let mut flash_count = 0;

    for _i in 0 .. 100 {
        for y in 1 .. height - 1 {
            for x in 1 .. width - 1 {
                fn iterate(map: &mut [Vec<Option<u32>>], x: usize, y: usize, flash_count: &mut u32) {
                    if let Some(a) = map[y][x] {
                        map[y][x] = Some(a + 1);
                        if a == 9 {
                            *flash_count += 1; 
                            let neighbors = [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
                            for n in neighbors {
                                iterate(map, n.0, n.1, flash_count);
                            }
                        }
                    }
                }
                iterate(map, x, y, &mut flash_count);
            }
        }

        for y in 1 .. height - 1 {
            for x in 1 .. width - 1 {
                if let Some(a) = map[y][x] {
                    if a > 9 {
                        map[y][x] = Some(0);
                    }
                }
            }
        }
    }
    println!("Part 1: {}", flash_count);
}

fn part_2(input: &Vec<Vec<Option<u32>>>) {
    let mut map_vec = input.clone();
    let height = map_vec.len();
    let width = if height != 0 { map_vec[0].len() } else { 0 };
    let map = map_vec.as_mut_slice();

    for i in 1 .. 10000 {
        for y in 1 .. height - 1 {
            for x in 1 .. width - 1 {
                fn iterate(map: &mut [Vec<Option<u32>>], x: usize, y: usize) {
                    if let Some(a) = map[y][x] {
                        map[y][x] = Some(a + 1);
                        if a == 9 {
                            let neighbors = [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
                            for n in neighbors {
                                iterate(map, n.0, n.1);
                            }
                        }
                    }
                }
                iterate(map, x, y);
            }
        }

        let mut flash_count = 0;
        for y in 1 .. height - 1 {
            for x in 1 .. width - 1 {
                if let Some(a) = map[y][x] {
                    if a > 9 {
                        map[y][x] = Some(0);
                        flash_count += 1;
                    }
                }
            }
        }
        if flash_count == (width - 2) * (height - 2) {
            println!("Part 2: {}", i);
            break;
        }
    }
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
