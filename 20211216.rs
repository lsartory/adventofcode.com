use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211216.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<bool> {
    input.first().unwrap_or(&String::new()).chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap_or(0))).collect::<String>().chars().map(|c| c == '1').collect::<Vec<bool>>()
}

/***********************************************/

fn part_1(input: &Vec<bool>) {
    let mut ptr = 0;
    let mut sum = 0;

    fn decode_packet(input: &Vec<bool>, ptr: &mut usize, sum: &mut usize) -> usize {
        let ptr_start = *ptr;
        let mut decode_value = |l: usize| { let val = (&input[*ptr .. *ptr + l]).iter().fold(0, |accum, x| (accum << 1) | if *x { 1 } else { 0 }); *ptr += l; val };

        *sum += decode_value(3);
        if decode_value(3) == 4 {
            while decode_value(5) >> 4 != 0 {}
        } else {
            if decode_value(1) == 0 {
                let mut l = decode_value(15);
                while l > 0 {
                    l -= decode_packet(input, ptr, sum);
                }
            } else {
                for _ in 0 .. decode_value(11) {
                    decode_packet(input, ptr, sum);
                }
            }
        }

        *ptr - ptr_start
    }

    decode_packet(input, &mut ptr, &mut sum);
    println!("Part 1: {}", sum);
}

fn part_2(input: &Vec<bool>) {
    let mut ptr = 0;

    fn decode_packet(input: &Vec<bool>, ptr: &mut usize) -> (usize, usize) {
        let mut ret = 0;
        let ptr_start = *ptr;
        let mut decode_value = |l: usize| { let val = (&input[*ptr .. *ptr + l]).iter().fold(0, |accum, x| (accum << 1) | if *x { 1 } else { 0 }); *ptr += l; val };

        decode_value(3);
        let t = decode_value(3);
        if t == 4 {
            loop {
                let x = decode_value(5);
                ret = (ret << 4) | (x & 0xF);
                if x >> 4 == 0 {
                    break;
                }
            }
        } else {
            ret = match t {
                1 => 1,
                2 => usize::MAX,
                _ => 0
            };
            let mut prev_val = ret;
            let mut operate = |val| {
                ret = match t {
                    0 => { ret + val },
                    1 => { ret * val },
                    2 => { ret.min(val) },
                    3 => { ret.max(val) },
                    5 => { if prev_val >  val { 1 } else { 0 } },
                    6 => { if prev_val  < val { 1 } else { 0 } },
                    7 => { if prev_val == val { 1 } else { 0 } },
                    _ => { ret }
                };
                prev_val = val;
            };
            if decode_value(1) == 0 {
                let mut l = decode_value(15);
                while l > 0 {
                    let (val, len) = decode_packet(input, ptr);
                    operate(val);
                    l -= len;
                }
            } else {
                for _ in 0 .. decode_value(11) {
                    operate(decode_packet(input, ptr).0);
                }
            }
        }

        (ret, *ptr - ptr_start)
    }

    println!("Part 2: {}", decode_packet(input, &mut ptr).0);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
