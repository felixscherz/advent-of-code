use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

fn antinode_at(this: &(i32, i32), other: &(i32, i32)) -> (i32, i32) {
    (
        this.0 + (other.0 - this.0) * 2,
        this.1 + (other.1 - this.1) * 2,
    )
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    // antennas grouped by frequency
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let height: i32 = input.lines().count().try_into().unwrap();
    let width: i32 = input.lines().next().unwrap().len().try_into().unwrap();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            match antennas.get_mut(&c) {
                None => {
                    antennas.insert(c, vec![(i.try_into().unwrap(), j.try_into().unwrap())]);
                }
                Some(positions) => positions.push((i.try_into().unwrap(), j.try_into().unwrap())),
            }
        }
    }
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for freq in antennas.keys() {
        let positions = antennas.get(freq).unwrap();
        for antenna in positions {
            for other_antenna in positions.iter().filter(|x| *x != antenna) {
                antinodes.insert(antinode_at(antenna, other_antenna));
            }
        }
    }

    let total = antinodes
        .iter()
        .filter(|x| x.0 < height && x.0 >= 0 && x.1 < width && x.1 >= 0)
        .count();

    println!("{}", total);
}
