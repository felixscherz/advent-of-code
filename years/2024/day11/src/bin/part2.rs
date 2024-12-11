use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Debug)]
struct Stone {
    pub number: u64,
}

impl Stone {
    fn new(number: u64) -> Self {
        Self { number }
    }

    fn blink(&self) -> Vec<Stone> {
        match self.number {
            0 => vec![Self::new(1)],
            n if n.to_string().len() % 2 == 0 => {
                let digits = n.to_string();
                let left: _ = digits[..digits.len() / 2].parse().unwrap();
                let right: _ = digits[digits.len() / 2..].parse().unwrap();
                vec![Self::new(left), Self::new(right)]
            }
            _ => vec![Self::new(self.number * 2024)],
        }
    }

    fn blink2(&self) -> (Self, Option<Self>) {
        match self.number {
            0 => (Self::new(1), None),
            n if n.to_string().len() % 2 == 0 => {
                let digits = n.to_string();
                let left: _ = digits[..digits.len() / 2].parse().unwrap();
                let right: _ = digits[digits.len() / 2..].parse().unwrap();
                (Self::new(left), Some(Self::new(right)))
            }
            _ => (Self::new(self.number * 2024), None),
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut stones: Vec<Stone> = vec![];
    for line in input.lines() {
        line.split_whitespace()
            .map(|x| stones.push(Stone::new(x.parse::<_>().unwrap())))
            .count();
    }

    let mut index: HashMap<u64, u64> = HashMap::new();
    for stone in &stones {
        let entry = index.entry(stone.number).or_insert(0);
        *entry = *entry + 1;
    }
    dbg!(&stones);
    dbg!(&index);
    // keep an index that keeps track how many stones were created with that number

    for i in 0..75 {
        // for key in index create a stone, create children, add those with multiplier to index and
        // remove old stone

        let current_index = index.clone();
        let stones: Vec<u64> = current_index.keys().cloned().collect();
        for number in stones {
            // eprintln!("blinking stone with number {}", &number);
            let stone = Stone::new(number);
            let count = &current_index.get(&number).unwrap().clone();
            // eprintln!("{} stones of that number", count);
            let children = stone.blink();
            // index.remove(&number); // what if a previous step created stones of that number
            let entry = index.get_mut(&number).unwrap();
            *entry = *entry - *count;

            for child in children {
                // eprintln!("created {}", &child.number);
                let entry = index.entry(child.number).or_insert(0);
                *entry = *entry + *count;
            }
        }
        // eprintln!("blinked {} times", i+1);
        // dbg!(&index);
    }

    println!("{}", index.values().sum::<u64>());
}
