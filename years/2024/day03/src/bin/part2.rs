use std::io::{stdin, BufReader, Read, Seek};

fn product(line: &str) -> i32 {
    let mut reader = BufReader::new(line.as_bytes());
    let mut sum: i32 = 0;
    let mut enabled = true;

    'outer: loop {
        match next_char(&mut reader) {
            '\0' => {
                return sum;
            }
            'd' => {
                if !(next_char(&mut reader) == 'o') {
                    continue;
                }
                match next_char(&mut reader) {
                    'n' => {
                        if !(next_char(&mut reader) == '\'') {
                            continue;
                        }
                        if !(next_char(&mut reader) == 't') {
                            continue;
                        }
                        if !(next_char(&mut reader) == '(') {
                            continue;
                        }
                        if !(next_char(&mut reader) == ')') {
                            continue;
                        }
                        enabled = false;
                    }
                    '(' => {
                        if !(next_char(&mut reader) == ')') {
                            continue;
                        }
                        enabled = true;
                        continue;
                    }
                    _ => continue,
                }
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

                if enabled {
                    sum = sum + left * right;
                }
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
    let sum = product(&input);
    println!("{}", sum);
}
