use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20211223.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

#[derive(Clone, Copy, Debug, PartialEq)]
enum Amphipod {
    A, B, C, D
}
impl Amphipod {
    fn cost(&self) -> usize {
        match self {
            Amphipod::A =>    1,
            Amphipod::B =>   10,
            Amphipod::C =>  100,
            Amphipod::D => 1000
        }
    }
    fn target_room(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3
        }
    }
    fn room_target(room: usize) -> Option<Self> {
        match room {
            0 => Some(Amphipod::A),
            1 => Some(Amphipod::B),
            2 => Some(Amphipod::C),
            3 => Some(Amphipod::D),
            _ => None
        }
    }

    //fn to_char(self) -> char {
    //    match self {
    //        Amphipod::A => 'A',
    //        Amphipod::B => 'B',
    //        Amphipod::C => 'C',
    //        Amphipod::D => 'D'
    //    }
    //}
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
             _  => None
        }
    }
}

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len()).map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>()).collect()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<Amphipod>> {
    transpose(input.iter().map(|line| line.chars().filter_map(Amphipod::from_char).collect::<Vec<_>>()).filter(|x| !x.is_empty()).collect::<Vec<_>>())
}

/***********************************************/

#[derive(Clone)]
struct State {
    hallway: [Option<Amphipod>; 11],
    rooms:   Vec<Vec<Option<Amphipod>>>
}
impl State {
    //fn display(&self) {
    //    let format_some = |x: &Option<Amphipod>| match x { Some(x) => x.to_char(), None => '.' };
    //    println!("#############");
    //    print!("#");
    //    for a in self.hallway {
    //        print!("{}", format_some(&a));
    //    }
    //    println!("#");
    //    for (i, a) in transpose(self.rooms.clone()).concat().iter().enumerate() {
    //        if i == 0 {
    //            print!("###");
    //        } else if i % 4 == 0 {
    //            print!("  #");
    //        }
    //        print!("{}#", format_some(a));
    //        if i == 3 {
    //            println!("##");
    //        } else if i % 4 == 3 {
    //            println!();
    //        }
    //    }
    //    println!("  #########");
    //}

    fn to_u128(&self) -> u128 {
        let mut ret = 0;
        let map_some = |x: &Option<Amphipod>| match *x {
            Some(Amphipod::A) => 1,
            Some(Amphipod::B) => 2,
            Some(Amphipod::C) => 3,
            Some(Amphipod::D) => 4,
            None              => 0
        };
        for a in transpose(self.rooms.clone()).concat().iter().rev() {
            ret = (ret << 3) | map_some(a);
        }
        for a in self.hallway.iter().rev() {
            ret = (ret << 3) | map_some(a);
        }
        ret
    }
    //fn from_u128(mut x: u128) -> Self {
    //    let map_int = |x| match x {
    //        1 => Some(Amphipod::A),
    //        2 => Some(Amphipod::B),
    //        3 => Some(Amphipod::C),
    //        4 => Some(Amphipod::D),
    //        _ => None
    //    };
    //    let mut hallway = [None; 11];
    //    for a in &mut hallway {
    //        *a = map_int(x & 0x7);
    //        x >>= 3;
    //    }
    //    let mut rooms = Vec::new();
    //    while x != 0 {
    //        rooms.push(map_int(x & 0x7));
    //        x >>= 3;
    //    }
    //    Self { hallway, rooms: transpose(rooms.chunks(4).map(|x| x.to_vec()).collect::<Vec<_>>()) }
    //}
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.to_u128() == other.to_u128()
    }
}
impl Eq for State {}

fn iterate(state: State, target_key: u128, known_states: &mut HashMap<u128, usize>, total_cost: usize) -> Option<usize> {
    let key = state.to_u128();
    if key == target_key {
        return Some(total_cost);
    }
    let entry = known_states.entry(key).or_insert(usize::MAX);
    if *entry <= total_cost {
        return None;
    }
    *entry = total_cost;
    let mut new_costs = Vec::new();

    const ALLOWED_HALLWAY: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
    let mut hallway_vec = ALLOWED_HALLWAY.iter().map(|i| (i, state.hallway[*i])).filter(|x| x.1.is_some()).map(|x| (x.0, x.1.unwrap())).collect::<Vec<_>>();
    hallway_vec.sort_unstable_by(|a, b| a.1.cost().cmp(&b.1.cost()));
    'hallway_loop: for (i, a) in hallway_vec {
        let target_hallway = (a.target_room() + 1) * 2;
        let mut cost = 0;
        let mut pos = *i;
        while pos != target_hallway {
            cost += 1;
            if pos < target_hallway {
                pos += 1;
            } else {
                pos -= 1;
            }
            if state.hallway[pos].is_some() {
                continue 'hallway_loop;
            }
        }
        let target_room = &state.rooms[a.target_room()];
        if !target_room.iter().any(|x| *x != None && *x != Some(a)) {
            let room_len = target_room.len();
            for j in 0 .. room_len {
                cost += 1;
                if j == room_len - 1 || target_room[j + 1].is_some() {
                    let mut next_state = state.clone();
                    next_state.hallway[*i] = None;
                    next_state.rooms[a.target_room()][j] = Some(a);
                    if let Some(new_cost) = iterate(next_state, target_key, known_states, total_cost + cost * a.cost()) {
                        new_costs.push(new_cost);
                    }
                }
            }
        }
    }

    for (i, r) in state.rooms.iter().enumerate() {
        let room_target = Amphipod::room_target(i).unwrap();
        if r.iter().any(|x| *x != None && *x != Some(room_target)) {
            for (j, a) in r.iter().enumerate() {
                if a.is_none() {
                    continue;
                }
                let a = a.unwrap();
                let target_pos = (a.target_room() + 1) * 2;
                'hallway_loop_2: for b in ALLOWED_HALLWAY {
                    let mut cost = j + 1;
                    let mut pos = (i + 1) * 2;
                    while pos != b {
                        cost += 1;
                        if pos < b {
                            pos += 1;
                        } else {
                            pos -= 1;
                        }
                        if state.hallway[pos].is_some() {
                            continue 'hallway_loop_2;
                        }
                        if pos == target_pos {
                            let target_room = &state.rooms[a.target_room()];
                            if !target_room.iter().any(|x| *x != None && *x != Some(a)) {
                                let mut tmp_cost = cost;
                                let room_len = target_room.len();
                                for k in 0 .. room_len {
                                    tmp_cost += 1;
                                    if k == room_len - 1 || target_room[k + 1].is_some() {
                                        let mut next_state = state.clone();
                                        next_state.rooms[i][j] = None;
                                        next_state.rooms[a.target_room()][k] = Some(a);
                                        if let Some(new_cost) = iterate(next_state, target_key, known_states, total_cost + tmp_cost * a.cost()) {
                                            new_costs.push(new_cost);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let mut next_state = state.clone();
                    next_state.hallway[pos] = Some(a);
                    next_state.rooms[i][j] = None;
                    if let Some(new_cost) = iterate(next_state, target_key, known_states, total_cost + cost * a.cost()) {
                        new_costs.push(new_cost);
                    }
                }
                break;
            }
        }
    }

    new_costs.iter().min().copied()
}

fn part_1(input: &[Vec<Amphipod>]) {
    let start_state = State { hallway: [None; 11], rooms: input.iter().map(|x| x.iter().map(|x| Some(*x)).collect::<Vec<_>>()).collect::<Vec<_>>() };
    println!("Part 1: {}", iterate(start_state, 79430515286867968, &mut HashMap::new(), 0).unwrap_or(usize::MAX));
}

fn part_2(input: &[Vec<Amphipod>]) {
    let mut start_state = State { hallway: [None; 11], rooms: input.iter().map(|x| x.iter().map(|x| Some(*x)).collect::<Vec<_>>()).collect::<Vec<_>>() };
    start_state.rooms[0].insert(1, Some(Amphipod::D));
    start_state.rooms[0].insert(2, Some(Amphipod::D));
    start_state.rooms[1].insert(1, Some(Amphipod::C));
    start_state.rooms[1].insert(2, Some(Amphipod::B));
    start_state.rooms[2].insert(1, Some(Amphipod::B));
    start_state.rooms[2].insert(2, Some(Amphipod::A));
    start_state.rooms[3].insert(1, Some(Amphipod::A));
    start_state.rooms[3].insert(2, Some(Amphipod::C));
    println!("Part 2: {}", iterate(start_state, 1332622991389601149485056, &mut HashMap::new(), 0).unwrap_or(usize::MAX));
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    part_1(&input);
    part_2(&input);
}
