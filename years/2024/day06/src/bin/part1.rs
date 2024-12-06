use std::{
    collections::HashSet,
    io::{stdin, Read},
};

struct Map {
    map: Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut map: Vec<Vec<char>> = vec![];
        for line in input.lines() {
            map.push(line.chars().collect());
        }
        let visited: HashSet<(usize, usize)> = HashSet::new();
        Map { map, visited }
    }

    fn get(&self, i: usize, j: usize) -> Option<char> {
        if let Some(row) = self.map.get(i) {
            if let Some(c) = row.get(j) {
                return Some(*c);
            }
        }
        None
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut char {
        self.map.get_mut(i).unwrap().get_mut(j).unwrap()
    }

    fn put(&mut self, i: usize, j: usize, c: char) {
        *self.get_mut(i, j) = c
    }

    fn find_guard(&self) -> (usize, usize) {
        for (i, row) in self.map.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                match c {
                    '<' | '^' | '>' | 'v' => return (i, j),
                    _ => continue,
                }
            }
        }
        panic!("no guard found")
    }

    fn step(&mut self) -> bool {
        let guard = self.find_guard();
        self.visited.insert(guard);
        let current_direction = self.get(guard.0, guard.1).unwrap();
        let candidate_field = match current_direction {
            '^' => (guard.0 - 1, guard.1),
            '>' => (guard.0, guard.1 + 1),
            'v' => (guard.0 + 1, guard.1),
            '<' => {
                if guard.1 == 0 {
                    return true;
                };
                (guard.0, guard.1 - 1)
            }
            _ => panic!("unkown direction"),
        };

        // check for end
        if self.get(candidate_field.0, candidate_field.1) == None {
            return true;
        };

        let new_direction = match self.get(candidate_field.0, candidate_field.1).unwrap() {
            '#' => match current_direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("unexpected direction"),
            },
            _ => current_direction,
        };

        let next_pos = match new_direction {
            '^' => (guard.0 - 1, guard.1),
            '>' => (guard.0, guard.1 + 1),
            'v' => (guard.0 + 1, guard.1),
            '<' => (guard.0, guard.1 - 1),
            _ => panic!("unkown direction"),
        };

        // move the guard now
        *self.get_mut(next_pos.0, next_pos.1) = new_direction;
        *self.get_mut(guard.0, guard.1) = 'X';
        false
    }

    fn unique(&self) -> usize {
        self.visited.iter().count()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut map = Map::from_str(&input);

    loop {
        if map.step() {
            break;
        }
    }

    println!("{}", map.unique());
}
