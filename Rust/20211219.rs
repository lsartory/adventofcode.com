use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211219.txt";
const MIN_OVERLAP_COUNT:usize = 12;

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}
impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
impl Point {
    fn manhattan(self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<Point>> {
    let mut ret = Vec::new();
    let mut list = Vec::new();
    for line in input.iter().skip(1) {
        if line.contains("scanner") {
            ret.push(list.clone());
            list.clear();
        } else {
            let coords: Vec<_> = line.split(',').map(|x| x.trim()).filter_map(|x| x.parse::<isize>().ok()).collect();
            if coords.len() == 3 {
                list.push(Point { x: coords[0], y: coords[1], z: coords[2] });
            }
        }
    }
    if !list.is_empty() {
        ret.push(list);
    }
    ret
}

/***********************************************/

type Matrix3x3 = [[isize; 3]; 3];
const ROTATION_MATRICES: [Matrix3x3; 24] = [
    [[ 1,  0,  0], [ 0,  1,  0], [ 0,  0,  1]],
    [[ 1,  0,  0], [ 0,  0, -1], [ 0,  1,  0]],
    [[ 1,  0,  0], [ 0, -1,  0], [ 0,  0, -1]],
    [[ 1,  0,  0], [ 0,  0,  1], [ 0, -1,  0]],
    [[ 0, -1,  0], [ 1,  0,  0], [ 0,  0,  1]],
    [[ 0,  0,  1], [ 1,  0,  0], [ 0,  1,  0]],
    [[ 0,  1,  0], [ 1,  0,  0], [ 0,  0, -1]],
    [[ 0,  0, -1], [ 1,  0,  0], [ 0, -1,  0]],
    [[-1,  0,  0], [ 0, -1,  0], [ 0,  0,  1]],
    [[-1,  0,  0], [ 0,  0, -1], [ 0, -1,  0]],
    [[-1,  0,  0], [ 0,  1,  0], [ 0,  0, -1]],
    [[-1,  0,  0], [ 0,  0,  1], [ 0,  1,  0]],
    [[ 0,  1,  0], [-1,  0,  0], [ 0,  0,  1]],
    [[ 0,  0,  1], [-1,  0,  0], [ 0, -1,  0]],
    [[ 0, -1,  0], [-1,  0,  0], [ 0,  0, -1]],
    [[ 0,  0, -1], [-1,  0,  0], [ 0,  1,  0]],
    [[ 0,  0, -1], [ 0,  1,  0], [ 1,  0,  0]],
    [[ 0,  1,  0], [ 0,  0,  1], [ 1,  0,  0]],
    [[ 0,  0,  1], [ 0, -1,  0], [ 1,  0,  0]],
    [[ 0, -1,  0], [ 0,  0, -1], [ 1,  0,  0]],
    [[ 0,  0, -1], [ 0, -1,  0], [-1,  0,  0]],
    [[ 0, -1,  0], [ 0,  0,  1], [-1,  0,  0]],
    [[ 0,  0,  1], [ 0,  1,  0], [-1,  0,  0]],
    [[ 0,  1,  0], [ 0,  0, -1], [-1,  0,  0]]
];

fn rotate(p: Point, m: &Matrix3x3) -> Point {
    Point {
        x: p.x * m[0][0] + p.y * m[0][1] + p.z * m[0][2],
        y: p.x * m[1][0] + p.y * m[1][1] + p.z * m[1][2],
        z: p.x * m[2][0] + p.y * m[2][1] + p.z * m[2][2]
    }
}

#[derive(Clone, Copy, Debug)]
struct Beacon {
    id:     usize,
    coords: Point
}

fn part_1(input: &[Vec<Point>]) -> Vec<Point> {
    let mut scanners = vec![Point { x: 0, y: 0, z: 0 }];

    let mut scanners_input = input.to_vec();
    let mut beacons = Vec::new();
    for (i, b) in scanners_input.swap_remove(0).iter().enumerate() {
        beacons.push(Beacon { id: i + 1, coords: *b });
    }

    'main_loop: while !scanners_input.is_empty() {
        let mut ref_offsets = Vec::new();
        let beacon_count = beacons.len();
        for i in 0 .. beacon_count {
            for j in 0 .. beacon_count {
                if i == j {
                    continue;
                }
                let offset = Point { x: beacons[j].coords.x - beacons[i].coords.x, y: beacons[j].coords.y - beacons[i].coords.y, z: beacons[j].coords.z - beacons[i].coords.z };
                ref_offsets.push((beacons[j].id, beacons[i].id, offset));
            }
        }

        let scanner_count = scanners_input.len();
        for s in 0 .. scanner_count {
            for m in ROTATION_MATRICES {
                let mut new_beacons = Vec::new();
                for b in &scanners_input[s] {
                    new_beacons.push(Beacon { id: 0, coords: rotate(*b, &m) });
                }

                let mut offsets = Vec::new();
                let beacon_count = new_beacons.len();
                for i in 0 .. beacon_count {
                    for j in 0 .. beacon_count {
                        if i == j {
                            continue;
                        }
                        let offset = new_beacons[j].coords - new_beacons[i].coords;
                        offsets.push((j, i, offset));
                    }
                }

                let mut dup_map = HashMap::new();
                for b in &ref_offsets {
                    *dup_map.entry(b.2).or_insert(0) += 1;
                }
                for b in &offsets {
                    *dup_map.entry(b.2).or_insert(0) += 1;
                }
                dup_map.retain(|_, v| *v > 1);
                if dup_map.len() / 2 >= MIN_OVERLAP_COUNT {
                    let mut offset_map = HashMap::new();
                    for p in dup_map {
                        let known = ref_offsets.iter().find(|x| x.2 == p.0).unwrap();
                        let new = offsets.iter().find(|x| x.2 == p.0).unwrap();
                        *offset_map.entry(beacons.iter().find(|x| x.id == known.0).unwrap().coords - new_beacons[new.0].coords).or_insert(0) += 1;
                        *offset_map.entry(beacons.iter().find(|x| x.id == known.1).unwrap().coords - new_beacons[new.1].coords).or_insert(0) += 1;
                    }
                    let offset = *offset_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
                    scanners.push(offset);
                    for b in new_beacons {
                        let coords = b.coords + offset;
                        if !beacons.iter().any(|x| x.coords == coords) {
                            beacons.push(Beacon { id: beacons.last().unwrap().id + 1, coords });
                        }
                    }

                    scanners_input.swap_remove(s);
                    continue 'main_loop;
                }
            }
        }
    }
    println!("Part 1: {}", beacons.len());

    scanners
}

fn part_2(input: &[Point]) {
    let mut max = 0;
    let scanner_count = input.len();
    for i in 0 .. scanner_count - 1 {
        for j in i + 1 .. scanner_count {
            max = max.max((input[j] - input[i]).manhattan());
        }
    }
    println!("Part 2: {}", max);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    let scanners = part_1(&input);
    part_2(&scanners);
}
