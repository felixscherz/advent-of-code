use std::io::{stdin, Read};

const XMAS: &str = "XMAS";
const PLACEHOLDER: char = '.';

fn number_of_xmas(i: usize, j: usize, chars: &Vec<Vec<char>>) -> u32 {
    // check whether we find an X and then go in every direction
    let start = get_char(i, j, chars);
    if !(start == 'A') {
        return 0;
    }
    let mut num = 0;

    let mut ne = false;
    let mut sw = false;
    let mut se = false;
    let mut nw = false;
    // north east
    if get_char(i + 1, j - 1, chars) == 'M' && get_char(i - 1, j + 1, chars) == 'S' {
        ne = true;
    }
    // south west
    if get_char(i - 1, j + 1, chars) == 'M' && get_char(i + 1, j - 1, chars) == 'S' {
        sw = true;
    }

    // south east
    if get_char(i - 1, j - 1, chars) == 'M' && get_char(i + 1, j + 1, chars) == 'S' {
        se = true;
    }

    // north west
    if get_char(i + 1, j + 1, chars) == 'M' && get_char(i - 1, j - 1, chars) == 'S' {
        nw = true;
    }

    if (ne || sw) && (se || nw) {
        return 1;
    }
    0
}

fn get_char(i: usize, j: usize, chars: &Vec<Vec<char>>) -> char {
    *chars
        .get(i)
        .expect(&format!("no now at i={}", i))
        .get(j)
        .expect(&format!("no char at i={}, j={}", i, j))
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut chars: Vec<Vec<char>> = vec![];

    let length = input.lines().next().unwrap().len();
    // padding on top
    for _ in 0..XMAS.len() {
        let row: Vec<char> = vec![PLACEHOLDER; length + (2 * XMAS.len())];
        chars.push(row);
    }

    for (_, line) in input.lines().enumerate() {
        let mut row: Vec<char> = vec![];
        // padding at the beginning
        vec![PLACEHOLDER; XMAS.len()]
            .iter()
            .map(|x| row.push(*x))
            .count();
        line.chars().map(|x| row.push(x)).count();
        // padding at the end
        vec![PLACEHOLDER; XMAS.len()]
            .iter()
            .map(|x| row.push(*x))
            .count();
        chars.push(row);
    }

    // padding on bottom
    for _ in 0..XMAS.len() {
        let row: Vec<char> = vec![PLACEHOLDER; length + (2 * XMAS.len())];
        chars.push(row);
    }

    let mut total = 0;

    for row in chars.clone() {
        let s: String = row.iter().collect();
        dbg!("{}", s);
    }
    for i in XMAS.len()..(chars.len() - XMAS.len()) {
        for j in XMAS.len()..(length + XMAS.len()) {
            total = total + number_of_xmas(i, j, &chars)
        }
    }

    println!("{}", total);
}