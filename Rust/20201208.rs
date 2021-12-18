use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201208.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(Clone, Default)]
struct Operation {
    opcode:     String,
    argument:   i64,
    exec_count: u32
}

fn parse_input(input: Vec<String>) -> Vec<Operation> {
    let mut ret = Vec::new();
    for line in input {
        let mut op: Operation = Default::default();
        for (i, part) in line.split(' ').map(|x| x.trim().to_string()).enumerate() {
            if i == 0 {
                op.opcode = part;
            } else {
                match part.parse() {
                    Ok(x) => { op.argument = x; ret.push(op); break; },
                        _ => {}
                }
            }
        }
    }
    ret
}

/***********************************************/

fn exec_program(mut input: Vec<Operation>) -> (bool, u64, usize, i64) {
    let mut count   = 0;
    let mut pointer = 0;
    let mut accum   = 0;
    loop {
        if pointer >= input.len() {
            return (true, count, pointer, accum);
        }
        let op = &mut input[pointer];
        if op.exec_count != 0 {
            return (false, count, pointer, accum);
        }
        match op.opcode.as_str() {
            "acc" => { pointer += 1; accum += op.argument; },
            "jmp" => { pointer += op.argument as usize },
                _ => { pointer += 1 }
        }
        op.exec_count += 1;
        count += 1;
    }
}

/***********************************************/

fn part_1(input: &Vec<Operation>) {
    let ret = exec_program(input.clone());
    if !ret.0 {
        println!("Restarted infinite loop after {} instructions at address {}; previous accum value was {}.", ret.1, ret.2, ret.3);
    } else {
        println!("Successfully completed execution after {} instructions at address {}; last accum value was {}.", ret.1, ret.2, ret.3);
    }
}

fn part_2(input: &Vec<Operation>) {
    for (i, op) in input.iter().enumerate() {
        match op.opcode.as_str() {
            "nop"|"jmp" => {
                let mut program = input.clone();
                program[i].opcode = (if op.opcode.as_str() == "nop" { "jmp" } else { "nop" }).to_string();
                let ret = exec_program(program);
                if ret.0 {
                    println!("Fixed instruction at address {}.", i);
                    println!("Successfully completed execution after {} instructions at address {}; last accum value was {}.", ret.1, ret.2, ret.3);
                    return;
                }
            },
            _ => {}
        }
    }
    println!("No solution could be found!");
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
