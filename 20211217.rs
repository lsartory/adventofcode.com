use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211217.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> ((i32, i32), (i32, i32)) {
    let coords = input.first().unwrap_or(&String::new()).split(&['=', '.', ','][..]).filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<_>>();
    if coords.len() >= 4 { ((coords[0].min(coords[1]), coords[2].min(coords[3])), (coords[0].max(coords[1]), coords[2].max(coords[3]))) } else { ((0, 0), (0, 0)) }
}

/***********************************************/

fn part_1(input: &((i32, i32), (i32, i32))) {
    println!("Part 1: {}", ((-input.0.1 - 1) * -input.0.1) / 2);
}

fn part_2(input: &((i32, i32), (i32, i32))) {
    let mut count = 0;
    for vel_y in (input.0.1 ..= -input.0.1).rev() {
        for vel_x in (0 ..= input.1.0).rev() {
            let mut vel = (vel_x, vel_y);
            let mut x = 0;
            let mut y = 0;
            let mut y_max = 0;
            loop {
                x += vel.0;
                y += vel.1;
                if vel.0 > 0 {
                    vel.0 -= 1;
                }
                vel.1 -= 1;
                y_max = y.max(y_max);
                if x > input.1.0 || y < input.0.1 {
                    break;
                }
                if x >= input.0.0 && x <= input.1.0 && y >= input.0.1 && y <= input.1.1 {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("Part 1: {}", count);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
