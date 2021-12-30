use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211222.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<Vec<isize>> {
    input.iter().map(|line| line.replace("on", "1").replace("off", "0").split(&[' ', '=', '.', ','][..]).filter_map(|x| x.parse::<isize>().ok()).collect::<Vec<_>>()).filter(|x| x.len() >= 7).collect::<Vec<_>>()
}

/***********************************************/

fn part_1(input: &[Vec<isize>]) {
    let mut cubes: HashMap<(isize, isize, isize), bool> = HashMap::new();
    for step in input {
        for x in step[1] ..= step[2] {
            if !(-50 ..= 50).contains(&x) {
                continue;
            }
            for y in step[3] ..= step[4] {
                if !(-50 ..= 50).contains(&y) {
                    continue;
                }
                for z in step[5] ..= step[6] {
                    if !(-50 ..= 50).contains(&z) {
                        continue;
                    }
                    *cubes.entry((x, y, z)).or_insert(false) = step[0] != 0;
                }
            }
        }
    }
    println!("Part 1: {}", cubes.iter().filter(|x| *x.1).count());
}

#[derive(Clone, Debug)]
struct Cuboid {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
    s: bool
}
impl Cuboid {
    fn intersection(&self, other: &Self) -> Option<Cuboid> {
        if (self.x.1 >= other.x.0) && (self.x.0 <= other.x.1) &&
           (self.y.1 >= other.y.0) && (self.y.0 <= other.y.1) &&
           (self.z.1 >= other.z.0) && (self.z.0 <= other.z.1) {
            Some(Cuboid {
                x: (self.x.0.max(other.x.0), self.x.1.min(other.x.1)),
                y: (self.y.0.max(other.y.0), self.y.1.min(other.y.1)),
                z: (self.z.0.max(other.z.0), self.z.1.min(other.z.1)),
                s: !other.s
            })
        } else {
            None
        }
    }
    fn count(&self) -> isize {
        let c = (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1);
        if self.s { c } else { -c }
    }
}

fn part_2(input: &[Vec<isize>]) {
    let mut cuboids: Vec<Cuboid> = Vec::new();
    for step in input {
        let cuboid = Cuboid {
            x: (step[1].min(step[2]), step[1].max(step[2])),
            y: (step[3].min(step[4]), step[3].max(step[4])),
            z: (step[5].min(step[6]), step[5].max(step[6])),
            s: step[0] != 0
        };
        for i in 0 .. cuboids.len() {
            if let Some(c) = cuboid.intersection(&cuboids[i]) {
                cuboids.push(c);
            }
        }
        if cuboid.s {
            cuboids.push(cuboid);
        }
    }
    println!("Part 2: {}", cuboids.iter().fold(0, |accum, x| accum + x.count()));
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
