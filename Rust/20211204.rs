use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211204.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> (Vec<u32>, Vec<(Vec<Vec<(u32, bool)>>, bool)>) {
    let numbers : Vec<_> = (input.first().unwrap_or(&"".to_string())).split(',').filter_map(|x| x.trim().parse().ok()).collect();
    let mut boards = Vec::new();
    let mut board = Vec::new();
    for line in input.iter().skip(1) {
        if line.is_empty() {
            if !board.is_empty() {
                boards.push((board.clone(), false));
                board.clear();
            }
            continue;
        }
        let row : Vec<_> = line.split(' ').filter(|&x| !x.is_empty()).map(|x| (x.trim().parse().unwrap_or(0), false)).collect();
        board.push(row);
    }
    if !board.is_empty() {
        boards.push((board.clone(), false));
    }
    (numbers, boards)
}

/***********************************************/

fn transpose<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..v[0].len()).map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>()).collect()
}

fn part_1(input: &(Vec<u32>, Vec<(Vec<Vec<(u32, bool)>>, bool)>)) {
    let numbers = &input.0;
    let mut boards = input.1.clone();
    let mut winner = (0, 0);
    'outer: for num in numbers {
        for (i, board) in boards.iter_mut().map(|x| &mut x.0).enumerate() {
            for x in board.iter_mut().flat_map(|x| x.iter_mut()).filter(|x| x.0 == *num) {
                x.1 = true;
            }
            for it in [board, &transpose(board)] {
                for row in it {
                    if row.iter().filter(|x| x.1).count() == row.len() {
                        winner = (*num, i);
                        break 'outer;
                    }
                }
            }
        }
    }
    let sum = boards[winner.1 as usize].0.iter().flat_map(|x| x.iter()).filter(|x| !x.1).map(|x| x.0).sum::<u32>();
    println!("Part 1: First winning number: {}, board: {}, sum: {}, product: {}", winner.0, winner.1, sum, sum * winner.0);
}

fn part_2(input: &(Vec<u32>, Vec<(Vec<Vec<(u32, bool)>>, bool)>)) {
    let numbers = &input.0;
    let mut boards = input.1.clone();
    let mut winner = (0, 0);
    for num in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.1 {
                continue;
            }
            for x in board.0.iter_mut().flat_map(|x| x.iter_mut()).filter(|x| x.0 == *num) {
                x.1 = true;
            }
            for it in [&board.0, &transpose(&board.0)] {
                for row in it {
                    if row.iter().filter(|x| x.1).count() == row.len() {
                        winner = (*num, i);
                        board.1 = true;
                    }
                }
            }
        }
    }
    let sum = boards[winner.1 as usize].0.iter().flat_map(|x| x.iter()).filter(|x| !x.1).map(|x| x.0).sum::<u32>();
    println!("Part 2: Last winning number:  {}, board: {}, sum: {}, product: {}", winner.0, winner.1, sum, sum * winner.0);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
