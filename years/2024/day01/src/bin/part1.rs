use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let numbers: Vec<&str> = lines.iter().flat_map(|x| x.split_whitespace()).collect();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    let mut num = numbers.iter();
    loop {
        if let Some(left_num_s) = num.next() {
            left.push(left_num_s.parse().unwrap());
        } else {
            break
        }
        let right_num: i32 = num.next().unwrap().parse().unwrap();
        right.push(right_num);
    }

    left.sort();
    right.sort();

    let distance: i32 = left.iter().zip(right.iter()).map(|(x,y)| (x-y).abs()).sum();
    println!("{}", distance);
}


