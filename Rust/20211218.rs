use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211218.txt";

/***********************************************/

#[derive(Clone, Debug)]
enum SnailFishNumber {
    Number(u32, usize),
    Pair(Box<SnailFishNumber>, Box<SnailFishNumber>)
}
impl std::fmt::Display for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SnailFishNumber::Number(v, _) => write!(f, "{}", v),
            SnailFishNumber::Pair(l, r)   => write!(f, "[{},{}]", l.as_ref(), r.as_ref())
        }
    }
}
impl SnailFishNumber {
    fn magnitude(&self) -> u64 {
        match self {
            SnailFishNumber::Number(v, _) => *v as u64,
            SnailFishNumber::Pair(l, r)   => l.magnitude() * 3 + r.magnitude() * 2
        }
    }
    fn reannotate(&mut self) {
        let mut index = 1;
        fn iterate(sfn: &mut SnailFishNumber, index: &mut usize) {
            match sfn {
                SnailFishNumber::Number(_, i) => {
                    *i = *index;
                    *index += 1;
                },
                SnailFishNumber::Pair(l, r) => {
                    iterate(l, index);
                    iterate(r, index);
                }
            }
        }
        iterate(self, &mut index);
    }
    fn add_at_index(&mut self, value: u32, index: usize) {
        fn iterate(sfn: &mut SnailFishNumber, index: usize, value: u32) -> bool {
            match sfn {
                SnailFishNumber::Number(v, i) => {
                    if *i == index {
                        *v += value;
                    }
                    *i >= index
                },
                SnailFishNumber::Pair(l, r) => {
                    iterate(l, index, value) || iterate(r, index, value)
                }
            }
        }
        if index > 0 {
            iterate(self, index, value);
        }
    }
}
impl std::ops::Add for SnailFishNumber {
    type Output = Self;
    fn add(self, rhs: SnailFishNumber) -> SnailFishNumber {
        fn explode(sfn: &mut SnailFishNumber) -> bool {
            fn iterate(sfn: &mut SnailFishNumber, level: usize) -> Option<((u32, usize), (u32, usize))> {
                match sfn {
                    SnailFishNumber::Number(_, _) => {},
                    SnailFishNumber::Pair(l, r) => {
                        if level >= 4 {
                            let ret_l = if let SnailFishNumber::Number(val, index) = **l { (val, index - 1) } else { (0, 0) };
                            let ret_r = if let SnailFishNumber::Number(val, index) = **r { (val, index + 1) } else { (0, 0) };
                            *sfn = SnailFishNumber::Number(0, 0);
                            return Some((ret_l, ret_r));
                        } else {
                            for i in [l, r] {
                                let ret = iterate(i, level + 1);
                                if ret.is_some() {
                                    return ret;
                                }
                            }
                        }
                    }
                }
                None
            }
            if let Some((l, r)) = iterate(sfn, 0) {
                sfn.add_at_index(l.0, l.1);
                sfn.add_at_index(r.0, r.1);
                return true;
            }
            false
        }
        fn split(sfn: &mut SnailFishNumber) -> bool {
            match sfn {
                SnailFishNumber::Number(v, _) => {
                    if *v >= 10 {
                        *sfn = SnailFishNumber::Pair(Box::new(SnailFishNumber::Number(*v / 2, 0)), Box::new(SnailFishNumber::Number((*v + 1) / 2, 0)));
                        return true;
                    }
                },
                SnailFishNumber::Pair(l, r) => {
                    if split(l) || split(r) {
                        return true;
                    }
                }
            }
            false
        }

        let mut ret = SnailFishNumber::Pair(Box::new(self), Box::new(rhs));
        //println!("after addition: {}", ret);
        loop {
            ret.reannotate();
            if explode(&mut ret) {
                //println!("after explode:  {}", ret);
                continue;
            }
            if split(&mut ret) {
                //println!("after split:    {}", ret);
                continue;
            }
            break;
        }
        //println!();
        ret
    }
}

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<SnailFishNumber> {
    let mut ret = Vec::new();
    for line in input {
        fn parse_elem(elem: &str) -> (usize, SnailFishNumber) {
            if &elem[0 ..= 0] != "[" {
                panic!("Syntax error in input, expected '[' not found");
            }
            let mut ptr = 1;
            let l = if &elem[ptr ..= ptr] == "[" {
                let ret = parse_elem(&elem[ptr ..]);
                ptr += ret.0 + 2;
                ret.1
            } else {
                let comma_pos = elem[ptr ..].find(',').expect("Syntax error in input, missing ','");
                let val = elem[ptr .. ptr + comma_pos].parse::<u32>().expect("Syntax error in input, invalid value");
                ptr += comma_pos + 1;
                SnailFishNumber::Number(val, 0)
            };
            let r = if &elem[ptr ..= ptr] == "[" {
                let ret = parse_elem(&elem[ptr ..]);
                ptr += ret.0 + 1;
                ret.1
            } else {
                let bracket_pos = elem[ptr ..].find(']').expect("Syntax error in input, missing ']'");
                let val = elem[ptr .. ptr + bracket_pos].parse::<u32>().expect("Syntax error in input, invalid value");
                ptr += bracket_pos;
                SnailFishNumber::Number(val, 0)
            };
            (ptr, SnailFishNumber::Pair(Box::new(l), Box::new(r)))
        }
        let mut num = parse_elem(&line).1;
        num.reannotate();
        ret.push(num);
    }
    ret
}

/***********************************************/

fn part_1(input: &[SnailFishNumber]) {
    println!("Part 1: {}", input.iter().cloned().reduce(|accum, x| accum + x).expect("Empty data set").magnitude());
}

fn part_2(input: &[SnailFishNumber]) {
    let mut max = 0;
    let len = input.len();
    for i in 0 .. len - 1 {
        for j in i + 1 .. len {
            max = max.max((input[i].clone() + input[j].clone()).magnitude());
        }
    }
    println!("Part 2: {}", max);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
