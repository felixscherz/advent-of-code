use std::fs;

fn check_record(record: &str) -> bool {
    let mut levels: Vec<i32> = record
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for i in 0..levels.len() {
        let mut dampened = levels.clone();
        dampened.remove(i);
        if check_levels(&dampened) {
            return true;
        }
    }

    false
}

fn check_levels(levels: &Vec<i32>) -> bool {
    let mut previous: Option<i32> = None;
    let mut ascending: Option<bool> = None;
    for (i, level) in levels.iter().enumerate() {
        if let Some(prev) = previous {
            let diff = level - prev;
            match ascending {
                None => {
                    ascending = Some(diff.is_positive());
                }
                Some(true) => {
                    if diff.is_negative() {
                        return false;
                    }
                }
                Some(false) => {
                    if diff.is_positive() {
                        return false;
                    }
                }
            }
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }
        }
        previous = Some(*level);
    }
    true
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let cnt = lines.iter().filter(|x| check_record(x)).count();
    println!("{}", cnt);
}
