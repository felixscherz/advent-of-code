use std::{
    collections::HashMap,
    fs::{self, File},
};

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();

    let mut adapters: Vec<u32> = binding.lines().map(|x| x.parse().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);

    let mut differences: HashMap<u32, u32> = HashMap::new();

    for i in 0..(adapters.len() - 1) {
        let diff = adapters.get(i + 1).unwrap() - adapters.get(i).unwrap();
        if let None = differences.get(&diff) {
            differences.insert(diff, 0);
        }
        let val = differences.get_mut(&diff).unwrap();
        *val = *val + 1
    }

    let total = (differences.get(&3).unwrap() + 1) *  differences.get(&1).unwrap();
    println!("{}", total);
}
