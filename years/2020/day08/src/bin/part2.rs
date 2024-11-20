use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct ParseOpError;

impl FromStr for Op {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let ins = parts.next().unwrap();
        let arg = parts.next().unwrap();
        let num: i32 = arg.parse().unwrap();

        match ins {
            "acc" => Ok(Op::Acc(num)),
            "jmp" => Ok(Op::Jmp(num)),
            "nop" => Ok(Op::Nop(num)),
            _ => Err(ParseOpError),
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut exchanged: HashSet<i32> = HashSet::new();
    loop {
        let mut visisted: HashSet<i32> = HashSet::new();
        let mut acc = 0;
        let mut pos: i32 = 0;
        let last_pos = lines.len();
        let mut changed = false;
        loop {
            let mut ins: Op = lines
                .get(TryInto::<usize>::try_into(pos).unwrap())
                .unwrap()
                .parse()
                .unwrap();
            if !changed {
                match ins {
                    Op::Acc(_) => (),
                    Op::Jmp(num) => {
                        if !exchanged.contains(&pos) {
                            exchanged.insert(pos);
                            changed = true;
                            ins = Op::Nop(num)
                        }
                    }
                    Op::Nop(num) => {
                        if !exchanged.contains(&pos) {
                            exchanged.insert(pos);
                            changed = true;
                            ins = Op::Jmp(num)
                        }
                    }
                }
            }
            if visisted.contains(&pos) {
                break;
            }
            visisted.insert(pos);
            match ins {
                Op::Acc(num) => {
                    pos += 1;
                    acc += num;
                }
                Op::Jmp(num) => pos += num,
                Op::Nop(num) => {
                    pos += 1;
                }
            }
            if pos == last_pos.try_into().unwrap() {
                println!("{}", acc);
                return;
            }
        }
    }
}

mod test {
    use crate::Op;

    #[test]
    fn name() {
        let op: Op = "nop +0".parse().unwrap();
        dbg!(op);
    }
}
