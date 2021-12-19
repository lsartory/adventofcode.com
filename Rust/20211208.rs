use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211208.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    let mut ret = Vec::new();
    for line in input {
        let parts: Vec<_> = line.split('|').map(|x| x.trim().to_string()).collect();
        let map_format = |x: &str| { let mut array = x.trim().chars().collect::<Vec<char>>(); array.sort_unstable(); array.iter().collect() };
        ret.push((parts[0].split(' ').map(map_format).collect(), parts[1].split(' ').map(map_format).collect()));
    }
    ret
}

/***********************************************/

fn part_1(input: &[(Vec<String>, Vec<String>)]) {
    println!("Part 1: {}", input.iter().map(|x| &x.1).flat_map(|x| x.iter()).filter(|x| { let len = x.len(); (2 ..= 4).contains(&len) || len == 7 }).count());
}

fn part_2(input: &[(Vec<String>, Vec<String>)]) {
    let mut sum = 0;
    fn contains(a: &str, b: &str) -> bool {
        let mut list: Vec<_> = b.to_string().chars().collect();
        for c in a.chars() {
            list.retain(|x| *x != c);
            if list.is_empty() {
                return true;
            }
        }
        false
    }
    for line in input {
        let mut numbers: [String; 10] = Default::default();
        let mut samples = line.0.clone();
        numbers[1] = samples.iter().find(|x| x.len() == 2).unwrap_or(&String::new()).to_string();
        samples.retain(|x| *x != numbers[1]);
        numbers[4] = (samples.iter().find(|x| x.len() == 4).unwrap_or(&String::new())).to_string();
        samples.retain(|x| *x != numbers[4]);
        numbers[7] = (samples.iter().find(|x| x.len() == 3).unwrap_or(&String::new())).to_string();
        samples.retain(|x| *x != numbers[7]);
        numbers[8] = (samples.iter().find(|x| x.len() == 7).unwrap_or(&String::new())).to_string();
        samples.retain(|x| *x != numbers[8]);
        numbers[3] = (samples.iter().find(|x| x.len() == 5 && contains(x, &numbers[1])).unwrap_or(&String::new())).to_string();
        samples.retain(|x| *x != numbers[3]);
        numbers[6] = (samples.iter().find(|x| x.len() == 6 && !contains(x, &numbers[1])).unwrap_or(&String::new())).to_string();
        samples.retain(|x| *x != numbers[6]);
        numbers[9] = (samples.iter().find(|x| x.len() == 6 && contains(x, &numbers[4])).unwrap_or(&String::new())).to_string();
        samples.retain(|x| *x != numbers[9]);
        numbers[0] = (samples.iter().find(|x| x.len() == 6).unwrap_or(&String::new())).to_string();
        samples.retain(|x| *x != numbers[0]);
        numbers[5] = (samples.iter().find(|x| x.len() == 5 && contains(&numbers[9], x)).unwrap_or(&String::new())).to_string();
        samples.retain(|x| *x != numbers[5]);
        numbers[2] = (samples.first().unwrap_or(&String::new())).to_string();
        sum += line.1.iter().fold(0, |accum, x| accum * 10 + numbers.iter().position(|y| y == x).unwrap_or(0));
    }
    println!("Part 2: {}", sum);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
