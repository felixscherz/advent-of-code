use std::{
    collections::HashMap,
    fs::{self, File},
};

// idea is to write a recursive function that explores all
// assume that there are not identical chargers first
// every adapter can branch off to at most 3 different other adapters (1, 2, or 3 jolts higher)
// write a function 'compatible' that returns a Vec with all compatible chargers

fn compatible(joltage: u32, adapters: &Vec<u32>) -> Vec<u32> {
    adapters
        .iter()
        .filter(|x| **x > joltage && **x - joltage <= 3)
        .map(|x| x.to_owned())
        .collect()
}

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();

    let mut adapters: Vec<u32> = binding.lines().map(|x| x.parse().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);

    let mut differences: HashMap<u32, u32> = HashMap::new();

    for i in 0..(adapters.len() - 1) {
        dbg!(compatible(*adapters.get(i).unwrap(), &adapters));
        let diff = adapters.get(i + 1).unwrap() - adapters.get(i).unwrap();
        if let None = differences.get(&diff) {
            differences.insert(diff, 0);
        }
        let val = differences.get_mut(&diff).unwrap();
        *val = *val + 1
    }

    let total = (differences.get(&3).unwrap() + 1) * differences.get(&1).unwrap();
    println!("{}", total);
}
