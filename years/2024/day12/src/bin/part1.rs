use std::{
    collections::HashSet,
    io::{stdin, Read},
    str::FromStr,
};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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
    let total_cost: usize = map.plots.iter().map(|x| map.cost(x)).sum();
    println!("{}", total_cost);
}
