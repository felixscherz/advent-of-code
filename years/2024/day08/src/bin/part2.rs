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

    let antinodes_at = |this: &(i32, i32), other: &(i32, i32)| -> Vec<(i32, i32)> {
        // direction normalized
        let mut difference: (f64, f64) = ((other.0 - this.0).into(), (other.1 - this.1).into());
        loop {
            let reduced_difference = (difference.0 / 2f64, difference.1 / 2f64);
            if reduced_difference.0.fract() != 0.0 || reduced_difference.1.fract() != 0.0 {
                break;
            }
            difference = reduced_difference
        }
        let direction: (i32, i32) = (difference.0.round() as i32, difference.1.round() as i32);

        // antinodes spawn beyond the location of the other antenna until they are out of bounds
        let mut antinodes: Vec<(i32, i32)> = vec![];

        let mut step = 0;
        loop {
            let antinode = (other.0 + direction.0 * step, other.1 + direction.1 * step);
            if !(antinode.0 < height && antinode.0 >= 0 && antinode.1 < width && antinode.1 >= 0) {
                break;
            }

            step = step + 1;
            antinodes.push(antinode);
        }
        antinodes
    };

    for freq in antennas.keys() {
        let positions = antennas.get(freq).unwrap();
        for antenna in positions {
            for other_antenna in positions.iter().filter(|x| *x != antenna) {
                for antinode in antinodes_at(antenna, other_antenna) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    // debugging
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if antinodes.contains(&(i.try_into().unwrap(), j.try_into().unwrap())) {
                eprint!("#");
            } else {
                eprint!("{}", c);
            }
        }
        eprintln!("");
    }


    let total = antinodes
        .iter()
        .count();

    println!("{}", total);
}
