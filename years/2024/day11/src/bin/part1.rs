use std::io::{stdin, Read};

#[derive(Debug)]
struct Stone {
    number: u64,
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
    for _ in 0..25 {
        stones = stones.iter().flat_map(|x| x.blink()).collect();

    }

    println!("{}", stones.len());

}
