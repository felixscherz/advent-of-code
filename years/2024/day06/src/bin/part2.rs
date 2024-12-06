use std::{
    collections::HashSet,
    io::{stdin, Read},
};

#[derive(Clone)]
struct Map {
    pub map: Vec<Vec<char>>,
    visited: HashSet<(usize, usize, char)>,
    loops: HashSet<(usize, usize)>,
    pub positions: HashSet<(usize, usize)>,
}

enum MapType {
    UNDETERMINED,
    FINITE,
    INFINITE,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut map: Vec<Vec<char>> = vec![];
        for line in input.lines() {
            map.push(line.chars().collect());
        }
        let visited: HashSet<(usize, usize, char)> = HashSet::new();
        let positions: HashSet<(usize, usize)> = HashSet::new();
        let loops: HashSet<(usize, usize)> = HashSet::new();
        Map {
            map,
            visited,
            loops,
            positions,
        }
    }

    fn width(&self) -> usize {
        self.map.get(0).unwrap().len()
    }

    fn height(&self) -> usize {
        self.map.len()
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

    fn check_loops(&self) -> bool {
        let guard = self.find_guard();
        let current_direction = self.get(guard.0, guard.1).unwrap();
        self.visited
            .contains(&(guard.0, guard.1, current_direction))
    }

    fn run(&mut self) -> MapType {
        loop {
            match self.step() {
                MapType::UNDETERMINED => (),
                x => return x,
            }
        }
    }

    fn add_obstacle_at(&mut self, i: usize, j: usize) -> bool {
        let guard = self.find_guard();
        if (i, j) == guard {
            return false;
        }
        match self.get(i, j).unwrap() {
            '#' => return false,
            _ => {
                *self.get_mut(i, j) = '#';
                return true;
            }
        }
    }

    fn step(&mut self) -> MapType {
        if self.check_loops() {
            return MapType::INFINITE;
        }
        let guard = self.find_guard();
        let current_direction = self.get(guard.0, guard.1).unwrap();
        self.visited.insert((guard.0, guard.1, current_direction));
        self.positions.insert((guard.0, guard.1));
        let candidate_field = match current_direction {
            '^' => {
                if guard.0 == 0 {
                    return MapType::FINITE;
                };
                (guard.0 - 1, guard.1)
            }
            '>' => (guard.0, guard.1 + 1),
            'v' => (guard.0 + 1, guard.1),
            '<' => {
                if guard.1 == 0 {
                    return MapType::FINITE;
                };
                (guard.0, guard.1 - 1)
            }
            _ => panic!("unkown direction"),
        };

        // check for end
        if self.get(candidate_field.0, candidate_field.1) == None {
            return MapType::FINITE;
        };

        let mut new_direction = match self.get(candidate_field.0, candidate_field.1).unwrap() {
            '#' => match current_direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("unexpected direction"),
            },
            _ => current_direction,
        };

        let mut next_pos = match new_direction {
            '^' => (guard.0 - 1, guard.1),
            '>' => (guard.0, guard.1 + 1),
            'v' => (guard.0 + 1, guard.1),
            '<' => (guard.0, guard.1 - 1),
            _ => panic!("unkown direction"),
        };

        // if next_pos is  abarrier we need to turn again
        if let Some(c) = self.get(next_pos.0, next_pos.1) {
            if c == '#' {
                new_direction = match new_direction {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("unexpected direction"),
                };

                next_pos = match new_direction {
                    '^' => (guard.0 - 1, guard.1),
                    '>' => (guard.0, guard.1 + 1),
                    'v' => (guard.0 + 1, guard.1),
                    '<' => (guard.0, guard.1 - 1),
                    _ => panic!("unkown direction"),
                };
            }
        }

        // move the guard now
        *self.get_mut(next_pos.0, next_pos.1) = new_direction;
        *self.get_mut(guard.0, guard.1) = match current_direction {
            '^' => 'u',
            '>' => 'r',
            'v' => 'd',
            '<' => 'l',
            _ => panic!("unexpected direction"),
        };
        MapType::UNDETERMINED
    }

    fn unique(&self) -> usize {
        self.visited.iter().count()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut map = Map::from_str(&input);

    // keep track of the guard's movement
    // try to "join" the track the guard was on previously
    // at every state, extend a line to the right and check if it joins an existing path
    // if one such block is found, mark its position and don't use it the next time
    // --> this only works for some loops I think
    //
    // better: build a check for infinite loops!
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    let mut checker = map.clone();
    checker.run();
    let on_path = checker.positions;
    eprintln!("number of pos to check: {}", on_path.len());

    for (i, j) in on_path.into_iter() {
        {
            let mut candidate = map.clone();
            if candidate.add_obstacle_at(i, j) {
                match candidate.run() {
                    MapType::INFINITE => {
                        positions.insert((i, j));
                    }
                    _ => (),
                }
            }
        }
    }

    println!("{}", positions.iter().count());
}
