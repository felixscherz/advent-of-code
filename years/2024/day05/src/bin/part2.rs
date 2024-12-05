use std::{
    collections::HashMap,
    io::{stdin, Read},
    mem,
};

fn find_leftmost(pages: &Vec<i32>, ordering: &HashMap<i32, Vec<i32>>) -> usize {
    // iterate through pages
    // for a given page, check if any of the other ones needs to come before it,
    // if not -> it is the lowest
    //
    //
    'outer: for (i, page) in pages.iter().enumerate() {
        for other in pages.iter().filter(|x| *x != page) {
            if let Some(rules) = ordering.get(other) {
                // if other needs to come before, page can't be the leftmost
                if rules.contains(page) {
                    continue 'outer;
                }
            }
        }
        return i;
    }
    panic!("couldn't determine lowest")
}

fn order(update: &mut Vec<i32>, ordering: &HashMap<i32, Vec<i32>>) {
    let mut ordered: Vec<i32> = vec![];
    // go through elements and find one that doesn't any other page before it
    // insert it
    loop {
        if update.len() == 0 {
            break;
        }
        let lowest_i = find_leftmost(update, ordering);
        ordered.push(update.remove(lowest_i));
    }
    *update = ordered;
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("reading stdin went wrong");

    let mut rules: Vec<&str> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];

    let mut lines = buffer.lines();

    loop {
        match lines.next() {
            Some("") => break,
            None => panic!("went too far"),
            Some(rule) => rules.push(rule),
        }
    }

    loop {
        match lines.next() {
            Some(update) => {
                let mut row: Vec<i32> = vec![];
                update
                    .split(",")
                    .map(|x| row.push(x.parse().unwrap()))
                    .count();
                updates.push(row);
            }
            None => break,
        }
    }

    let mut ordering: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rules {
        let mut parts = rule.split("|");
        let left: i32 = parts
            .next()
            .expect("left integer")
            .parse()
            .expect("could not parse to int");
        let right: i32 = parts
            .next()
            .expect("right integer")
            .parse()
            .expect("could not parse to int");
        match ordering.get_mut(&left) {
            Some(order) => order.push(right),
            None => {
                ordering.insert(left, vec![right]);
            }
        }
    }

    let mut valid_updates = 0;
    let mut total = 0;
    dbg!(&ordering);
    for update in updates {
        'pages: for (i, page) in update.iter().enumerate() {
            dbg!(page);
            for j in i + 1..update.len() {
                if ordering.contains_key(&update.get(j).unwrap())
                    && ordering
                        .get(&update.get(j).unwrap())
                        .unwrap()
                        .contains(page)
                {
                    let mut update = update.clone();
                    dbg!("before order", &update);
                    order(&mut update, &ordering);
                    dbg!(&update);
                    let middle = update.get(update.len() / 2).unwrap();
                    total = total + middle;
                    valid_updates = valid_updates + 1;
                    break 'pages;
                }
            }
            dbg!("skipping", &update);
            continue 'pages;
        }
    }

    println!("{}", total);
}
