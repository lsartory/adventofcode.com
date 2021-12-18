use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201212.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

enum Direction {
    Unknown,
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}
impl Default for Direction {
    fn default() -> Self { Direction::Unknown }
}

#[derive(Default)]
struct Step {
    direction: Direction,
    amount:    f64
}

fn parse_input(input: Vec<String>) -> Vec<Step> {
    let mut ret = Vec::new();
    for line in input {
        let mut step: Step = Default::default();
        let mut chars = line.chars();
        step.direction = match chars.next() {
            Some(c) => match c {
                'N' => Direction::North,
                'S' => Direction::South,
                'E' => Direction::East,
                'W' => Direction::West,
                'L' => Direction::Left,
                'R' => Direction::Right,
                'F' => Direction::Forward,
                 _  => continue
            }
            _ => continue
        };
        match chars.as_str().parse() { Ok(x) => step.amount = x, _ => continue };
        ret.push(step);
    }
    ret
}

/***********************************************/

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64
}
impl Default for Point {
    fn default() -> Self { Self { x: 0.0, y: 0.0 } }
}
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { x: self.x + rhs.x, y: self.y + rhs.y };
    }
}
impl std::ops::Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Point {
    fn cartesian_to_polar(&self) -> Self {
        Self { x: (self.x * self.x + self.y * self.y).sqrt(), y: self.y.atan2(self.x) }
    }
    fn polar_to_cartesian(&self) -> Self {
        Self { x: self.x * self.y.cos(), y: self.x * self.y.sin() }
    }
    fn round(&mut self) {
        *self = Self { x: self.x.round(), y: self.y.round() };
    }
    fn manhattan_distance(&self) -> u64 {
        (self.x.round().abs() + self.y.round().abs()) as u64
    }
}

/***********************************************/

fn part_1(input: &Vec<Step>) {
    let mut pos: Point = Default::default();
    let mut angle: f64 = -90.0;
    for step in input {
        match step.direction {
            Direction::North   => pos.y += step.amount,
            Direction::South   => pos.y -= step.amount,
            Direction::East    => pos.x += step.amount,
            Direction::West    => pos.x -= step.amount,
            Direction::Left    => angle -= step.amount,
            Direction::Right   => angle += step.amount,
            Direction::Forward => pos += Point { x: step.amount as f64, y: angle.to_radians() }.polar_to_cartesian(),
            _ => continue
        }
        pos.round();
    }
    println!("Final position Manhattan distance (part 1): {}", pos.manhattan_distance());
}

fn part_2(input: &Vec<Step>) {
    let mut waypoint = Point { x: 10.0, y: 1.0 };
    let mut ship     = Point { x:  0.0, y: 0.0 };
    for step in input {
        match step.direction {
            Direction::North   => waypoint.y += step.amount,
            Direction::South   => waypoint.y -= step.amount,
            Direction::East    => waypoint.x += step.amount,
            Direction::West    => waypoint.x -= step.amount,
            Direction::Left    => {
                let mut polar_waypoint = waypoint.cartesian_to_polar();
                polar_waypoint.y += step.amount.to_radians();
                waypoint = polar_waypoint.polar_to_cartesian();
            },
            Direction::Right   => {
                let mut polar_waypoint = waypoint.cartesian_to_polar();
                polar_waypoint.y -= step.amount.to_radians();
                waypoint = polar_waypoint.polar_to_cartesian();
            },
            Direction::Forward => ship += waypoint * step.amount,
            _ => continue
        }
        waypoint.round();
        ship.round();
    }
    println!("Final position Manhattan distance (part 2): {}", ship.manhattan_distance());
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
