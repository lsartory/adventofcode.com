use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211224.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(Clone, Copy, PartialEq, Debug)]
enum Register {
    W,
    X,
    Y,
    Z
}

#[derive(Clone, Copy, Debug)]
enum Source {
    Reg(Register),
    Imm(i64)
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Inp(Register),
    Add(Register, Source),
    Mul(Register, Source),
    Div(Register, Source),
    Mod(Register, Source),
    Eql(Register, Source)
}

fn parse_input(input: Vec<String>) -> Vec<Operation> {
    let mut ret = Vec::new();
    for line in input {
        let parts = line.split(' ').map(|x| x.trim().to_string()).collect::<Vec<_>>();
        let get_register = |r: &String| {
            match r.chars().nth(0) {
                Some('w') => Some(Register::W),
                Some('x') => Some(Register::X),
                Some('y') => Some(Register::Y),
                Some('z') => Some(Register::Z),
                        _ => None
            }
        };
        let get_source = |x: &String| {
            if let Ok(i) = x.parse::<i64>() {
                Some(Source::Imm(i))
            } else {
                if let Some(r) = get_register(x) { Some(Source::Reg(r)) } else { None }
            }
        };
        let rhs = if parts.len() >= 3 { get_source(&parts[2]) } else { None };
        let lhs = if parts.len() >= 2 { get_register(&parts[1]) } else { None };
        let op  = if parts.len() >= 1 {
            match (parts[0].as_str(), lhs, rhs) {
                ("inp", Some(lhs), None)      => { Some(Operation::Inp(lhs))      },
                ("add", Some(lhs), Some(rhs)) => { Some(Operation::Add(lhs, rhs)) },
                ("mul", Some(lhs), Some(rhs)) => { Some(Operation::Mul(lhs, rhs)) },
                ("div", Some(lhs), Some(rhs)) => { Some(Operation::Div(lhs, rhs)) },
                ("mod", Some(lhs), Some(rhs)) => { Some(Operation::Mod(lhs, rhs)) },
                ("eql", Some(lhs), Some(rhs)) => { Some(Operation::Eql(lhs, rhs)) },
                                            _ => { None }
            }
        } else { None };
        if let Some(op) = op {
            ret.push(op);
        }
    }
    ret
}

/***********************************************/

fn part_1(input: &[Operation]) {
    for i in (11111111111111i64 ..= 99999999999999).rev() {
        let model = i.to_string();
        let mut model_iter = model.chars();

        let mut w = 0;
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        for op in input {
            let (lhs, rhs) = match op {
                Operation::Inp(r) => { (r, Source::Imm(model_iter.next().expect("Model number is too short").to_digit(10).expect("Invalid digit in model number") as i64)) },
                Operation::Add(lhs, rhs) => { (lhs, *rhs) },
                Operation::Mul(lhs, rhs) => { (lhs, *rhs) },
                Operation::Div(lhs, rhs) => { (lhs, *rhs) },
                Operation::Mod(lhs, rhs) => { (lhs, *rhs) },
                Operation::Eql(lhs, rhs) => { (lhs, *rhs) }
            };
            let read_reg = |r| {
                match r {
                    Register::W => w,
                    Register::X => x,
                    Register::Y => y,
                    Register::Z => z
                }
            };
            let lhs_val = read_reg(*lhs);
            let rhs_val = match rhs {
                Source::Imm(x) => x,
                Source::Reg(r) => read_reg(r)
            };
            let val = match op {
                Operation::Inp(_)    => { rhs_val },
                Operation::Add(..) => { lhs_val + rhs_val },
                Operation::Mul(..) => { lhs_val * rhs_val },
                Operation::Div(..) => { lhs_val / rhs_val },
                Operation::Mod(..) => { lhs_val % rhs_val },
                Operation::Eql(..) => { if lhs_val == rhs_val { 1 } else { 0 }}
            };
            match lhs {
                Register::W => { w = val; },
                Register::X => { x = val; },
                Register::Y => { y = val; },
                Register::Z => { z = val; }
            };
        }
        if z == 0 {
            println!("Part 1: {}", i);
            break;
        }

        break; // Obviously the brute force option is way too slow, so don't try it...
    }

    // Solution based on Kamiel de Visser's analysis
    // https://github.com/kemmel-dev/AdventOfCode2021/tree/master/day24
    let mut digit = [0; 14];
    let mut stack = Vec::new();
    let check  = input.iter().filter_map(|x| if let Operation::Add(lhs, rhs) = x { if let Source::Imm(val) = rhs { (*lhs == Register::X).then(|| val) } else { None } } else { None }).collect::<Vec<_>>();
    let offset = input.iter().filter_map(|x| if let Operation::Add(lhs, rhs) = x { if let Source::Imm(val) = rhs { (*lhs == Register::Y).then(|| val) } else { None } } else { None }).collect::<Vec<_>>().chunks(3).map(|x| x[2]).collect::<Vec<_>>();
    for (i, p) in check.iter().zip(offset).enumerate() {
        if **p.0 > 0 {
            stack.push((i, p.1));
        } else {
            let x = stack.pop().unwrap();
            let diff = *x.1 + **p.0;
            if diff < 0 {
                digit[x.0] = 9;
                digit[i] = 9 + diff;
            } else {
                digit[i] = 9;
                digit[x.0] = 9 - diff;
            }
        }
    }
    println!("Part 1: {}", digit.iter().filter_map(|x| char::from_digit(*x as u32, 10)).collect::<String>());
}

fn part_2(input: &[Operation]) {
    let mut digit = [0; 14];
    let mut stack = Vec::new();
    let check  = input.iter().filter_map(|x| if let Operation::Add(lhs, rhs) = x { if let Source::Imm(val) = rhs { (*lhs == Register::X).then(|| val) } else { None } } else { None }).collect::<Vec<_>>();
    let offset = input.iter().filter_map(|x| if let Operation::Add(lhs, rhs) = x { if let Source::Imm(val) = rhs { (*lhs == Register::Y).then(|| val) } else { None } } else { None }).collect::<Vec<_>>().chunks(3).map(|x| x[2]).collect::<Vec<_>>();
    for (i, p) in check.iter().zip(offset).enumerate() {
        if **p.0 > 0 {
            stack.push((i, p.1));
        } else {
            let x = stack.pop().unwrap();
            let diff = *x.1 + **p.0;
            if diff < 0 {
                digit[i] = 1;
                digit[x.0] = 1 - diff;
            } else {
                digit[x.0] = 1;
                digit[i] = 1 + diff;
            }
        }
    }
    println!("Part 2: {}", digit.iter().filter_map(|x| char::from_digit(*x as u32, 10)).collect::<String>());
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
