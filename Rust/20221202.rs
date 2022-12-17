use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::cmp::Ordering;

/***********************************************/

const INPUT_FILE:&str = "20221202.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissor
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Hand::Rock    => Some(match other { Hand::Rock => Ordering::Equal,   Hand::Paper => Ordering::Less,    Hand::Scissor => Ordering::Greater }),
            Hand::Paper   => Some(match other { Hand::Rock => Ordering::Greater, Hand::Paper => Ordering::Equal,   Hand::Scissor => Ordering::Less    }),
            Hand::Scissor => Some(match other { Hand::Rock => Ordering::Less,    Hand::Paper => Ordering::Greater, Hand::Scissor => Ordering::Equal   })
        }
    }
}
impl Hand {
    fn from_str(x: &str) -> Option<Self> {
        match x {
            "A" => Some(Hand::Rock),
            "B" => Some(Hand::Paper),
            "C" => Some(Hand::Scissor),
             _  => None
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Hand::Rock    => Hand::Scissor,
            Hand::Paper   => Hand::Rock,
            Hand::Scissor => Hand::Paper
        }
    }
    fn loses_against(&self) -> Self {
        match self {
            Hand::Rock    => Hand::Paper,
            Hand::Paper   => Hand::Scissor,
            Hand::Scissor => Hand::Rock
        }
    }

    fn value(&self) -> u32 {
        match self {
            Hand::Rock    => 1,
            Hand::Paper   => 2,
            Hand::Scissor => 3
        }
    }
}

enum Strategy {
    Lose,
    Tie,
    Win
}
impl Strategy {
    fn from_str(x: &str) -> Option<Self> {
        match x {
            "X" => Some(Strategy::Lose),
            "Y" => Some(Strategy::Tie),
            "Z" => Some(Strategy::Win),
             _  => None
        }
    }

    fn value(&self) -> u32 {
        match self {
            Strategy::Lose => 0,
            Strategy::Tie  => 3,
            Strategy::Win  => 6
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<(Hand, Strategy)> {
    let tuple_opt = |x| match x { (Some(a), Some(b)) => Some((a, b)), _ => None };
    input.iter().filter_map(|x| x.split_once(' ')).collect::<Vec<(&str, &str)>>().iter().filter_map(|x| tuple_opt((Hand::from_str(x.0), Strategy::from_str(x.1)))).collect::<Vec<(Hand, Strategy)>>()
}

/***********************************************/

fn part_1(input: &[(Hand, Strategy)]) {
    let score = input.iter().map(|x| (&x.0, match x.1 { Strategy::Lose => &Hand::Rock, Strategy::Tie => &Hand::Paper, Strategy::Win => &Hand::Scissor })).fold(0, |acc, x|
        acc + x.1.value() + (
            if x.1 > x.0 {
                Strategy::Win
            } else if x.1 == x.0 {
                Strategy::Tie
            } else {
                Strategy::Lose
            }).value()
    );
    println!("Part 1: {:?}", score);
}

fn part_2(input: &[(Hand, Strategy)]) {
    let score = input.iter().fold(0, |acc, x|
        acc + x.1.value() + match x.1 {
            Strategy::Lose => x.0.wins_against().value(),
            Strategy::Tie  => x.0.value(),
            Strategy::Win  => x.0.loses_against().value()
        }
    );
    println!("Part 2: {}", score);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).unwrap_or_else(|_| panic!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
