use std::{
    collections::VecDeque, fs::File, io::{BufRead, BufReader}
    
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let numbers: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();
    let preamble = 25;

    let mut numbers = numbers.iter();


    let mut available: VecDeque<i64> = VecDeque::new();

    for _ in 0..preamble {
        available.push_front(*numbers.next().unwrap());
    }

    loop {
        let candidate = *numbers.next().expect("reached end of numbers");

        let mut found = false;

        // dbg!(&available);
        // println!("trying {}", candidate);

        for i in 0..preamble {
            if found {
                break;
            }
            for j in 0..preamble {
                if found {
                    break;
                }
                if i == j {
                    continue;
                };
                if available.get(i).unwrap() + available.get(j).unwrap() == candidate {
                    found = true;
                }
            }
        }

        if !found {
            println!("{}", candidate);
            break;
        }

        available.push_front(candidate);
        available.pop_back();
    }
}
