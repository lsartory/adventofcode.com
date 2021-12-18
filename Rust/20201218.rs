use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201218.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn solve(expr: &str) -> u64 {
    let expr = format!("0+{}", expr);
    let len = expr.len();
    let mut result = 0;
    let mut r = 1;
    let mut l = r;
    loop {
        if l >= len {
            break;
        }
        let rhs = if &expr[l + 1 ..= l + 1] == "(" {
            let mut level = 0;
            for (i, c) in expr[l + 1 ..].chars().enumerate() {
                if c == '(' {
                    level += 1;
                } else if c == ')' {
                    level -= 1;
                }
                if level == 0 {
                    r += i + 2;
                    break;
                }
            }
            solve(&expr[l + 2 .. r - 1])
        } else {
            r = match expr[l + 1 ..].find(&['+', '*'][..]) { Some(x) => l + x + 1, _ => len };
            match expr[l + 1 .. r].parse::<u64>() { Ok(x) => x, _ => break }
        };
        match &expr[l ..= l] {
            "+" => result += rhs,
            "*" => result *= rhs,
             _  => break
        }
        l = r;
    }
    result
}

fn part_1(input: &Vec<String>) {
    let mut total = 0;
    for line in input {
        let result = solve(&line.replace(' ', ""));
        total += result;
    }
    println!("Sum of all results (part 1): {}", total);
}

fn part_2(input: &Vec<String>) {
    let mut total = 0;
    for line in input {
        let mut expr = line.replace(' ', "");
        let mut ptr = 0;
        loop {
            ptr += match expr[ptr + 1 ..].find('+') { Some(x) => x + 1, _ => break };
            let mut level = 0;
            if &expr[ptr - 1 ..= ptr - 1] == ")" {
                for (i, c) in expr[0 .. ptr].chars().rev().enumerate() {
                    if c == ')' {
                        level += 1;
                    } else if c == '(' {
                        level -= 1;
                    }
                    if level == 0 {
                        expr.insert(ptr - i, '(');
                        break;
                    }
                }
            } else {
                let l = match expr[0 .. ptr].rfind(&['+', '*'][..]) { Some(x) => x + 1, _ => 0 };
                expr.insert(l, '(');
            }
            ptr += 1;
            level = 0;
            if &expr[ptr + 1 ..= ptr + 1] == "(" {
                for (i, c) in expr[ptr + 1 ..].chars().enumerate() {
                    if c == '(' {
                        level += 1;
                    } else if c == ')' {
                        level -= 1;
                    }
                    if level == 0 {
                        expr.insert(ptr + i + 1, ')');
                        break;
                    }
                }
            } else {
                let r = match expr[ptr + 1 ..].find(&['+', '*'][..]) { Some(x) => ptr + x + 1, _ => expr.len() };
                expr.insert(r, ')');
            }
            ptr += 1;
        }
        let result = solve(&expr);
        total += result;
    }
    println!("Sum of all results (part 2): {}", total);
}

/***********************************************/

fn main() {
    let input = read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
