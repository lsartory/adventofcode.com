use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201211.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(Clone, PartialEq)]
enum SeatState {
    Free,
    Occupied,
    Unavailable
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<SeatState>> {
    let mut ret = Vec::new();
    for line in input {
        let mut seats = Vec::new();
        seats.push(SeatState::Free);
        for seat in line.chars() {
            match seat {
                'L' => seats.push(SeatState::Free),
                 _  => seats.push(SeatState::Unavailable)
            }
        }
        seats.push(SeatState::Free);
        ret.push(seats);
    }
    if ret.len() > 0 {
        let width = ret[0].len();
        ret.insert(0, vec![SeatState::Free; width]);
        ret.push(vec![SeatState::Free; width]);
    }
    ret
}

/***********************************************/

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
{
    (0 .. v[0].len()).map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>()).collect()
}

fn print_state(seats: &Vec<Vec<SeatState>>) {
    for row in seats.iter().take(seats.len() - 1).skip(1) {
        for col in row.iter().take(row.len() - 1).skip(1) {
            print!("{}", match col {
                   SeatState::Free        => 'L',
                   SeatState::Occupied    => '#',
                   SeatState::Unavailable => '.'
            });
        }
        println!();
    }
    println!();
}

/***********************************************/

fn iterate_part_1(seats: &Vec<Vec<SeatState>>) -> Vec<Vec<SeatState>> {
    let mut ret = Vec::new();
    for i in seats.windows(3) {
        let mut row = Vec::new();
        row.push(SeatState::Free);
        for j in transpose(i.to_vec()).windows(3) {
            let mut group = j.concat();
            let seat = group.remove(group.len() / 2);
            let occupied_count = group.iter().filter(|x| **x == SeatState::Occupied).count();
            row.push(match seat {
                SeatState::Free     => if occupied_count == 0 { SeatState::Occupied } else { SeatState::Free },
                SeatState::Occupied => if occupied_count <= 3 { SeatState::Occupied } else { SeatState::Free },
                                  _ => SeatState::Unavailable
            });
        }
        row.push(SeatState::Free);
        ret.push(row);
    }
    if ret.len() > 0 {
        let width = ret[0].len();
        ret.insert(0, vec![SeatState::Free; width]);
        ret.push(vec![SeatState::Free; width]);
    }
    ret
}

fn part_1(input: &Vec<Vec<SeatState>>) {
    let mut iteration = 0;
    let mut state = input.clone();
    loop {
        let new_state = iterate_part_1(&state);
        if new_state == state {
            break;
        }
        state = new_state;
        iteration += 1;
    }
    println!("Iteration #{}:", iteration);
    print_state(&state);
    println!("Occupied seats (part 1): {}", state.iter().map(|x| x.iter().filter(|y| **y == SeatState::Occupied).count()).sum::<usize>());
}

/***********************************************/

fn iterate_part_2(seats: &Vec<Vec<SeatState>>) -> Vec<Vec<SeatState>> {
    let mut ret = Vec::new();
    for (x, row) in seats.iter().take(seats.len() - 1).skip(1).enumerate() {
        let mut new_row = Vec::new();
        new_row.push(SeatState::Free);
        for (y, seat) in row.iter().take(row.len() - 1).skip(1).enumerate() {
            let pos = (x + 1, y + 1);
            let vectors = vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];
            let mut occupied_count = 0;
            if *seat != SeatState::Unavailable {
                for dir in &vectors {
                    let mut offset = 1;
                    loop {
                        match seats[pos.0 + offset * dir.0 as usize][pos.1 + offset * dir.1 as usize] {
                            SeatState::Free     => { break; },
                            SeatState::Occupied => { occupied_count += 1; break; },
                                              _ => {}
                        }
                        offset += 1;
                    }
                }
            }
            new_row.push(match *seat {
                SeatState::Free     => if occupied_count == 0 { SeatState::Occupied } else { SeatState::Free },
                SeatState::Occupied => if occupied_count <= 4 { SeatState::Occupied } else { SeatState::Free },
                                  _ => SeatState::Unavailable
            });
        }
        new_row.push(SeatState::Free);
        ret.push(new_row);
    }
    if ret.len() > 0 {
        let width = ret[0].len();
        ret.insert(0, vec![SeatState::Free; width]);
        ret.push(vec![SeatState::Free; width]);
    }
    ret
}

fn part_2(input: &Vec<Vec<SeatState>>) {
    let mut state = input.clone();
    loop {
        let new_state = iterate_part_2(&state);
        if new_state == state {
            break;
        }
        state = new_state;
    }
    println!("Occupied seats (part 2): {}", state.iter().map(|x| x.iter().filter(|y| **y == SeatState::Occupied).count()).sum::<usize>());
}

/***********************************************/

fn main() {
    let input = parse_input(&read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
