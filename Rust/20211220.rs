use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211220.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut algorithm = Vec::new();
    let mut image = Vec::new();
    for (i, line) in input.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut row: Vec<_> = line.split("").filter(|&x| !x.is_empty()).collect::<Vec<_>>().iter().map(|x| x.trim() == "#").collect();
        if i == 0 {
            algorithm = row;
        } else {
            let mut extended_row = vec![false; 2];
            extended_row.append(&mut row);
            extended_row.append(&mut vec![false; 2]);
            image.push(extended_row);
        }
    }
    if !image.is_empty() {
        let width = image[0].len();
        for _ in 0 .. 2 {
            image.insert(0, vec![false; width]);
            image.push(vec![false; width]);
        }
    }
    (algorithm, image)
}

/***********************************************/

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len()).map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>()).collect()
}

fn part_1(input: &(Vec<bool>, Vec<Vec<bool>>)) {
    //fn print_image(image: &[Vec<bool>]) {
    //    for i in image {
    //        println!("{}", i.iter().map(|x| if *x { '#' } else { '.' }).collect::<String>());
    //    }
    //    println!();
    //}
    let mut image = input.1.clone();
    //print_image(&image);
    for n in 0 .. 2 {
        let toggle = input.0[0] && (n & 1) == 0;
        let mut new_image = Vec::new();
        for i in image.windows(3) {
            let mut new_row = vec![toggle; 2];
            for j in transpose(i.to_vec()).windows(3) {
                let index = transpose(j.to_vec()).concat().iter().fold(0, |accum, x| (accum << 1) | if *x { 1 } else { 0 });
                new_row.push(input.0[index]);
            }
            new_row.append(&mut vec![toggle; 2]);
            new_image.push(new_row);
        }
        if !new_image.is_empty() {
            let width = new_image[0].len();
            for _ in 0 .. 2 {
                new_image.insert(0, vec![toggle; width]);
                new_image.push(vec![toggle; width]);
            }
        }
        //print_image(&new_image);
        image = new_image.clone();
    }
    println!("Part 1: {}", image.concat().iter().filter(|x| **x).count());
}

fn part_2(input: &(Vec<bool>, Vec<Vec<bool>>)) {
    let mut image = input.1.clone();
    for n in 0 .. 50 {
        let toggle = input.0[0] && (n & 1) == 0;
        let mut new_image = Vec::new();
        for i in image.windows(3) {
            let mut new_row = vec![toggle; 2];
            for j in transpose(i.to_vec()).windows(3) {
                let index = transpose(j.to_vec()).concat().iter().fold(0, |accum, x| (accum << 1) | if *x { 1 } else { 0 });
                new_row.push(input.0[index]);
            }
            new_row.append(&mut vec![toggle; 2]);
            new_image.push(new_row);
        }
        if !new_image.is_empty() {
            let width = new_image[0].len();
            for _ in 0 .. 2 {
                new_image.insert(0, vec![toggle; width]);
                new_image.push(vec![toggle; width]);
            }
        }
        image = new_image.clone();
    }
    println!("Part 2: {}", image.concat().iter().filter(|x| **x).count());
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
