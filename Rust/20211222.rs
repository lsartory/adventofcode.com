use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211222.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<Vec<isize>> {
    input.iter().map(|line| line.replace("on", "1").replace("off", "0").split(&[' ', '=', '.', ','][..]).filter_map(|x| x.parse::<isize>().ok()).collect::<Vec<_>>()).filter(|x| x.len() >= 7).collect::<Vec<_>>()
}

/***********************************************/

fn part_1(input: &[Vec<isize>]) {
    let mut cubes: HashMap<(isize, isize, isize), bool> = HashMap::new();
    for step in input {
        for x in step[1] ..= step[2] {
            if !(-50 ..= 50).contains(&x) {
                continue;
            }
            for y in step[3] ..= step[4] {
                if !(-50 ..= 50).contains(&y) {
                    continue;
                }
                for z in step[5] ..= step[6] {
                    if !(-50 ..= 50).contains(&z) {
                        continue;
                    }
                    *cubes.entry((x, y, z)).or_insert(false) = step[0] != 0;
                }
            }
        }
    }
    println!("Part 1: {}", cubes.iter().filter(|x| *x.1).count());
}

//fn part_2(input: &[Vec<isize>]) {
//}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    //part_2(&input);
}
