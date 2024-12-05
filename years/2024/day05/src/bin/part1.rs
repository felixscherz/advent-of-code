use std::{
    collections::HashMap,
    io::{stdin, Read},
};

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

    for rule in &rules {
        dbg!(rule);
    }

    for update in &updates {
        dbg!(update);
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
    'updates: for update in updates {
        for (i, page) in update.iter().enumerate() {
            for j in i + 1..update.len() {
                if ordering.contains_key(&update.get(j).unwrap())
                    && ordering
                        .get(&update.get(j).unwrap())
                        .unwrap()
                        .contains(page)
                {
                    continue 'updates;
                }
            }
        }
        let middle = update.get(update.len() / 2).unwrap();
        total = total + middle;
        valid_updates = valid_updates + 1;
    }

    println!("{}", total);
}
