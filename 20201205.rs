use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

/***********************************************/

const INPUT_FILE:&str = "20201205.txt";

/***********************************************/

fn read_input(filename: &str) -> Result<Vec<String>> {
    BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .map(|line| line?.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

/***********************************************/

fn parse_input(input: Vec<String>) -> Vec<u16> {
    let mut ret = input
        .iter()
        .map(|l| l.chars().map(|c| match c {
                               'F' => '0', 'B' => '1',
                               'L' => '0', 'R' => '1',
                               _ => c
                               }).collect::<String>())
        .filter_map(|l| u16::from_str_radix(l.as_str(), 2).ok())
        .collect::<Vec<u16>>();
    ret.sort_unstable();
    ret
}

/***********************************************/

fn part_1(input: &Vec<u16>) {
    println!("Highest seat ID: {}", match input.iter().last() { Some(max) => max.to_string(), _ => "<Error>".to_string()});
}

fn part_2(input: &Vec<u16>) {
    let min = match input.iter().next() { Some(min) => *min, _ => 0 };
    for (i, x) in input.iter().enumerate() {
        if (x - min) as usize != i {
            println!("Missing seat ID: {}", i + min as usize);
            break;
        }
    }
}

/***********************************************/

fn main() {
    let input = parse_input(read_input(INPUT_FILE).expect(&format!("Could not read {}", INPUT_FILE)));
    println!("Total records: {}", input.len());
    part_1(&input);
    part_2(&input);
}

/***********************************************/

/* Bonus: Zsh / Bash one-liner!
LIST=$(for i in $(tr 'FBLR' '0101' < 20201205.txt | sort); do echo $((2#$i)); done); MAX=$(tail -1 <<< $LIST); echo -e "Total records: $(wc -l <<< $LIST)\nHighest seat ID: $MAX\nMissing seat ID: $(diff <(cat <<< $LIST) <(seq $(head -1 <<< $LIST) $MAX) | tail -1 | cut -d' ' -f2)"
*/
