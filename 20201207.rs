use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201207.txt";
const MY_BAG:&str = "shiny gold";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn get_hash(x: String) -> u64 {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

#[derive(Clone, Default, PartialEq)]
struct Bag {
    id: u64,
    contents: Vec<(u32, u64)>
}
impl Bag {
    fn get_sub_count(&self, bags: &Vec<Bag>) -> u32 {
        let mut count = 1;
        for x in &self.contents {
            count += x.0 * find_bag(bags, x.1).get_sub_count(bags);
        }
        if self.id == 0 {
            0
        } else {
            count
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Bag> {
    let mut ret = Vec::new();
    for line in input {
        let mut bag: Bag = Default::default();
        let mut contents = String::new();
        for (i, part) in line.split("bags contain").map(|part| part.trim().replace("bags", "bag").to_string()).enumerate() {
            if i == 0 {
                bag.id = get_hash(part);
            } else {
                contents = match part.strip_suffix("bag.") { Some(x) => x.to_string(), _ => part };
            }
        }
        for element in contents.split("bag, ") {
            let mut count = 0;
            let mut id = 0;
            for (i, part) in element.splitn(2, ' ').map(|part| part.trim().to_string()).enumerate() {
                if i == 0 {
                    count = match part.parse::<u32>() { Ok(x) => x, _ => 0 };
                } else {
                    id = get_hash(part.trim().to_string());
                }
            }
            if count != 0 {
                bag.contents.push((count, id));
            }
        }
        ret.push(bag);
    }
    ret
}

/***********************************************/

fn find_bag(bags: &Vec<Bag>, id: u64) -> Bag {
    match bags.iter().find(|bag| bag.id == id) { Some(bag) => bag.clone(), _ => Default::default() }
}

fn find_all_bags_containing(bags: &Vec<Bag>, id: u64) -> Vec<u64> {
    let mut ret: Vec<u64> = bags.iter().filter(|bag| bag.contents.iter().filter(|b| b.1 == id).count() != 0).map(|b| b.id).collect();
    for b in ret.clone().iter() {
        ret.append(&mut find_all_bags_containing(bags, *b));
    }
    ret.sort();
    ret.dedup();
    ret
}

fn part_1(input: &Vec<Bag>) {
    println!("{} bag colors can contain {} bags.", find_all_bags_containing(input, get_hash(MY_BAG.to_string())).iter().count(), MY_BAG);
}

fn part_2(input: &Vec<Bag>) {
    println!("A {} bag must contain {} other bags.", MY_BAG, find_bag(input, get_hash(MY_BAG.to_string())).get_sub_count(input) - 1);
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}
