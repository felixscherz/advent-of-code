use std::io::{stdin, BufReader, Read, Seek};

fn product(line: &str) -> i32 {
    let mut reader = BufReader::new(line.as_bytes());
    let mut sum: i32 = 0;

    'outer: loop {
        match next_char(&mut reader) {
            '\0' => {
                return sum;
            }
            'm' => {
                if !(next_char(&mut reader) == 'u') {
                    continue;
                }
                if !(next_char(&mut reader) == 'l') {
                    continue;
                }
                if !(next_char(&mut reader) == '(') {
                    continue;
                }
                let mut left_s = String::new();
                loop {
                    match next_char(&mut reader) {
                        n @ '0'..='9' => left_s.push(n),
                        ',' => break,
                        _ => continue 'outer,
                    }
                }
                let mut right_s = String::new();
                loop {
                    match next_char(&mut reader) {
                        n @ '0'..='9' => right_s.push(n),
                        ')' => break,
                        _ => continue 'outer,
                    }
                }
                let left: i32 = left_s.parse().unwrap();
                let right: i32 = right_s.parse().unwrap();

                sum = sum + left * right;
            }
            _ => (),
        }
    }
}

fn next_char<'a>(stream: &mut (dyn 'a + Read)) -> char {
    let mut buf: [u8; 1] = [0];
    if stream.read(&mut buf).unwrap() == 1 {
        return buf[0] as char;
    }
    '\0'
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let sum: i32 = input.lines().map(|x| product(x)).sum();
    println!("{}", sum);
}
