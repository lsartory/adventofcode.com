use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211209.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<Vec<u32>> {
    let mut map = Vec::new();
    for line in input {
        let mut row: Vec<u32> = line.split("").filter(|&x| !x.is_empty()).collect::<Vec<&str>>().iter().map(|x| match x.trim().parse() { Ok(x) => x, _ => 0 }).collect();
        row.insert(0, 9);
        row.push(9);
        map.push(row);
    }
    if map.len() > 0 {
        let width = map[0].len();
        map.insert(0, vec![9; width]);
        map.push(vec![9; width]);
    }
    map
}

/***********************************************/

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len()).map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>()).collect()
}

fn part_1(input: &Vec<Vec<u32>>) {
    let mut min = Vec::new();
    for i in input.windows(3) {
        for j in transpose(i.to_vec()).windows(3) {
            let group = j.concat();
            let center = group[group.len() / 2];
            if center < group.iter().enumerate().map(|x| if x.0 & 1 != 0 { *x.1 } else { 9 }).fold(9, |accum, x| if accum < x { accum } else { x }) {
                min.push(center);
            }
        }
    }
    println!("Part 1: {}", min.iter().map(|x| x + 1).sum::<u32>());
}

fn part_2(input: &Vec<Vec<u32>>) {
    let map_vec = input.clone();
    let height = map_vec.len();
    let width = if height != 0 { map_vec[0].len() } else { 0 };
    let map = map_vec.as_slice();

    let mut basin_num = 1;
    let mut map_vec2 = input.clone();
    let map2 = map_vec2.as_mut_slice();
    for i in map2.iter_mut() {
        for j in i {
            *j = 0;
        }
    }

    for y in 0 .. height {
        for x in 0 .. width {
            if map[y][x] == 9 {
                map2[y][x] = 0;
            } else {
                let neighbors: Vec<u32> = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)].iter().map(|n| map[n.1][n.0]).collect();
                if map[y][x] < match neighbors.iter().min() { Some(x) => *x, _ => 9 } {
                    fn iterate(map: &[Vec<u32>], map2: &mut [Vec<u32>], x: usize, y: usize, basin_num: u32) {
                        map2[y][x] = basin_num;
                        let neighbors = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];
                        for n in neighbors {
                            if map[n.1][n.0] != 9 && map2[n.1][n.0] == 0 {
                                iterate(map, map2, n.0, n.1, basin_num);
                            }
                        }
                    }
                    iterate(map, map2, x, y, basin_num);
                    basin_num += 1;
                }
            }
        }
    }

    let mut basins = Vec::new();
    for i in 1 .. basin_num {
        basins.push(map_vec2.iter().flat_map(|x| x.iter()).filter(|x| **x == i).count());
    }
    basins.sort_unstable();
    println!("Part 2: {}", basins.iter().rev().take(3).fold(1, |accum, x| accum * x));
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
