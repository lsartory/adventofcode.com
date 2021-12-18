use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201213.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn part_1(input: &Vec<String>) {
    let timestamp = match input[0].parse::<u64>() { Ok(x) => x, _ => 0 };
    let lines: Vec<u64> = input[1].split(',').filter_map(|bus| bus.parse().ok()).collect();
    let delay: Vec<u64> = lines.iter().map(|bus| ((timestamp / bus) + 1) * bus).collect();
    let earliest = match delay.iter().enumerate().min_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs)).map(|(i, _)| i) { Some(x) => x, _ => 0 };
    println!("The earliest bus after {} is bus #{} with a delay of {} minutes.", timestamp, lines[earliest], delay[earliest]);
    println!("Part 1: {}", (delay[earliest] - timestamp) * lines[earliest]);
}

fn part_2(input: &Vec<String>) {
    let lines: Vec<(u64, u64)> = input[1].split(',').map(|bus| bus.parse()).enumerate().filter(|x| x.1.is_ok()).map(|x| (x.0 as u64, x.1.unwrap())).collect();
    let base_period = lines[0].1;
    let mut offset: u64 = 0;
    let mut period: u64 = 1;
    for bus in lines.iter().skip(1) {
        let mut cnt: u64 = 1;
        while (base_period * (offset + period * cnt) + bus.0) % bus.1 != 0 {
            cnt += 1;
        }
        offset = offset + cnt * period;
        period *= bus.1;
    }
    println!("Part 2: {}", offset * base_period);
}

/***********************************************/

fn main() {
    let input = read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
