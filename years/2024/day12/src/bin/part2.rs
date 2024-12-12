use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
    str::FromStr,
};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
static HORIZONTAL_DIRECTIONS: [(i32, i32); 2] = [(0, 1), (0, -1)];
static VERTICAL_DIRECTIONS: [(i32, i32); 2] = [(1, 0), (-1, 0)];

enum Side {
    HORIZONTAL(i32), // store the row
    VERTICAL(i32),   // store the column
}

struct Garden {
    plants: Vec<Vec<char>>,
    pub plots: Vec<HashSet<(usize, usize)>>,
    assigned: HashSet<(usize, usize)>,
}

#[derive(Debug)]
struct GardenParseError;

impl Garden {
    fn new(plants: Vec<Vec<char>>) -> Self {
        Self {
            plants,
            plots: vec![],
            assigned: HashSet::new(),
        }
    }

    fn get(&self, pos: &(usize, usize)) -> Option<char> {
        if let Some(row) = self.plants.get(pos.0) {
            return row.get(pos.1).copied();
        }
        None
    }

    fn neighbours(&self, start: &(usize, usize)) -> Vec<(usize, usize)> {
        DIRECTIONS
            .iter()
            .map(|x| {
                (
                    (start.0 as i32 + x.0) as usize,
                    (start.1 as i32 + x.1) as usize,
                )
            })
            .filter(|x| self.get(x).is_some())
            .collect()
    }

    fn neighbours_of_type(&self, start: &(usize, usize), plant_type: char) -> Vec<(usize, usize)> {
        self.neighbours(start)
            .iter()
            .filter(|x| {
                if let Some(plant) = self.get(*x) {
                    return plant == plant_type;
                } else {
                    return false;
                }
            })
            .copied()
            .collect()
    }

    fn number_of_sides(&self, start: &(usize, usize)) -> usize {
        let plant_type = self.get(start).unwrap();
        DIRECTIONS
            .iter()
            .map(|x| {
                (
                    (start.0 as i32 + x.0) as usize,
                    (start.1 as i32 + x.1) as usize,
                )
            })
            .filter(|x| {
                if let Some(other) = self.get(x) {
                    return other != plant_type;
                } else {
                    return true;
                }
            })
            .count()
    }

    fn sides(&self, plot: &HashSet<(usize, usize)>) -> HashSet<(i32, i32)> {
        let plant_type = self.get(plot.iter().next().unwrap()).unwrap();
        plot.iter()
            .flat_map(|start| {
                DIRECTIONS
                    .iter()
                    .map(|x| ((start.0 as i32 + x.0), (start.1 as i32 + x.1))) // make the step
                    .filter(|x| {
                        // if the other is a plant of the same type, there is no side
                        if let Some(other) = self.get(&(x.0 as usize, x.1 as usize)) {
                            return other != plant_type;
                        } else {
                            return true;
                        }
                    })
            })
            .collect()
    }

    fn reachable(&self, start: &(i32, i32), end: &(i32, i32)) -> bool {
        for dir in DIRECTIONS {
            if start.0 + dir.0 == end.0 && (start.1 + dir.1) == end.1 {
                return true;
            }
        }
        false
    }

    fn combine_sides(&self, sides: &HashSet<(i32, i32)>) -> usize {
        let mut assigned: HashSet<(i32, i32)> = HashSet::new();

        let mut sections: Vec<HashSet<(i32, i32)>> = vec![];

        for side in sides {
            // if side element was already grouped, ignore it
            if assigned.contains(side) {
                continue;
            }
            assigned.insert(*side);

            // section will contain all elements that belong to a side
            let mut section: HashSet<(i32, i32)> = HashSet::new();
            section.insert(*side);

            // used for bfs
            let mut current: HashSet<(i32, i32)> = HashSet::new();
            current.insert(*side);
            // find other sides that belong to that one
            loop {
                let mut new: HashSet<(i32, i32)> = HashSet::new();

                dbg!(&current);
                // for every element in the current section we are checking the neighbours
                for pos in &current {
                    // go through all side elements and find those that are reachable
                    for other in sides {
                        if pos == other {
                            eprintln!("continue because same");
                            continue;
                        }

                        if !self.reachable(pos, other) {
                            eprintln!("continue because {}, {}, not reachable", other.0, other.1);
                            continue;
                        }

                        if assigned.contains(other) {
                            eprintln!("continue because already assigned");
                            continue;
                        }
                        eprintln!("found {}, {}", other.0, other.1);

                        new.insert(*other);
                        section.insert(*other);
                        assigned.insert(*other);
                    }
                }
                if new.is_empty() {
                    dbg!("no new");
                    break;
                }
                current = new;
            }
            sections.push(section);
        }
        sections.len()
    }

    fn cost(&self, plot: &HashSet<(usize, usize)>) -> usize {
        let sides: usize = plot.iter().map(|x| self.number_of_sides(x)).sum();
        plot.len() * sides
    }

    fn find_plot(&mut self, start: &(usize, usize)) {
        if self.assigned.contains(start) {
            return;
        }
        let mut plot: HashSet<(usize, usize)> = HashSet::new();
        plot.insert(*start);
        let plant_type = self.get(start).unwrap();
        let mut current: Vec<(usize, usize)> = vec![*start];

        loop {
            let mut new: Vec<(usize, usize)> = vec![];
            for pos in &current {
                let neighbours = self.neighbours_of_type(pos, plant_type);
                new.extend(neighbours.iter().filter(|x| !plot.contains(x)));
                plot.extend(&neighbours);
            }
            if new.is_empty() {
                break;
            }
            current = new;
        }
        self.assigned.extend(&plot);
        self.plots.push(plot);
    }

    fn find_plots(&mut self) {
        for i in 0..self.plants.len() {
            for j in 0..self.plants[0].len() {
                if self.assigned.contains(&(i, j)) {
                    continue;
                }
                self.find_plot(&(i, j));
            }
        }
    }
}

impl FromStr for Garden {
    type Err = GardenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut plants = vec![];
        for line in s.lines() {
            plants.push(line.chars().collect());
        }
        Ok(Self::new(plants))
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // approach:
    // iterate through the plants in the garden
    // start a BFS for every plant, collecting all associated plants and creating an area
    //
    let mut map = input.parse::<Garden>().unwrap();
    map.find_plots();
    eprintln!("found {} plots", map.plots.len());
    let mut cost = 0;
    for plot in &map.plots {
        eprintln!(
            "plot {} has {} sides",
            map.get(plot.iter().next().unwrap()).unwrap(),
            map.combine_sides(&map.sides(&plot))
        );
        cost = cost + map.combine_sides(&map.sides(&plot)) * plot.len();
    }
    // let total_cost: usize = map.plots.iter().map(|x| map.cost(x)).sum();
    dbg!(map.reachable(&(1,1), &(1,2)));

    for plot in &map.plots {
        println!(
            "plot {} has {} sides",
            map.get(plot.iter().next().unwrap()).unwrap(),
            map.combine_sides(&map.sides(&plot))
        );
    }
    println!("{}", cost);
}
