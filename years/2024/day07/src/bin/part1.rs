use std::{
    io::{stdin, Read},
    str::FromStr,
};

struct Equation {
    pub goal: u64,
    parts: Vec<u64>,
}

#[derive(Debug)]
struct ParseEquationError;

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts_s = s.split(":");
        let goal = parts_s.next().unwrap().parse().unwrap();
        let mut parts = vec![];
        for part_s in parts_s.next().unwrap().split_whitespace() {
            parts.push(part_s.parse().unwrap());
        }

        Ok(Self { goal, parts })
    }
}
enum Op {
    ADD,
    MULTIPLY,
}

impl Equation {
    fn valid(&self) -> bool {
        // can this be done in a reduce like way?
        // for a given acc and the rest of the vec c
        fn combine(goal: u64, acc: u64, op: Op, mut operands: Vec<u64>) -> bool {
            if operands.is_empty() {
                return acc == goal;
            }

            let new_acc = match op {
                Op::ADD => acc + operands.remove(0),
                Op::MULTIPLY => acc * operands.remove(0),
            };

            if new_acc > goal {
                return false;
            }

            return combine(goal, new_acc, Op::ADD, operands.clone())
                || combine(goal, new_acc, Op::MULTIPLY, operands.clone());
        }

        let mut operands = self.parts.clone();
        let acc = operands.remove(0);
        combine(self.goal, acc, Op::ADD, operands.clone())
            || combine(self.goal, acc, Op::MULTIPLY, operands.clone())
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let result: u64 = input
        .lines()
        .map(|x| x.parse::<Equation>().unwrap())
        .filter(|x| x.valid())
        .map(|x| x.goal)
        .sum();
    println!("{}", result);
}
