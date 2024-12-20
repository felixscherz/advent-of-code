use std::{
    io::{stdin, Read},
    str::FromStr,
};

#[derive(Clone, Debug)]
enum GridPoint {
    Robot,
    Empty,
    Box,
    Wall,
}

#[derive(Debug)]
struct ParseGridPointError;
impl FromStr for GridPoint {
    type Err = ParseGridPointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let gridpoint = match s {
            "#" => GridPoint::Wall,
            "@" => GridPoint::Robot,
            "." => GridPoint::Empty,
            "O" => GridPoint::Box,
            _ => return Err(ParseGridPointError),
        };
        Ok(gridpoint)
    }
}

enum Direction {
    Top,
    Right,
    Left,
    Down,
}

#[derive(Debug)]
struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s {
            "^" => Direction::Top,
            "v" => Direction::Down,
            ">" => Direction::Right,
            "<" => Direction::Left,
            _ => return Err(ParseDirectionError),
        };
        Ok(dir)
    }
}

struct Warehouse {
    grid: Vec<Vec<GridPoint>>,
}

impl Warehouse {
    fn new(grid: Vec<Vec<GridPoint>>) -> Self {
        Self { grid }
    }

    fn robot_pos(&self) -> (usize, usize) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, entry) in row.iter().enumerate() {
                match entry {
                    GridPoint::Robot => return (i, j),
                    _ => continue,
                }
            }
        }
        panic!("did not find robot")
    }

    fn sum_box_coordinates(&self) -> usize {
        let mut sum = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, entry) in row.iter().enumerate() {
                match entry {
                    GridPoint::Box => sum = 100 * i + j + sum,
                    _ => continue,
                }
            }
        }
        sum
    }

    fn move_robot(&mut self, direction: Direction) {
        let robot_pos = self.robot_pos();
        self.move_to(robot_pos, &direction);
    }

    /// returns whether the block was able to move
    fn move_to(&mut self, pos: (usize, usize), direction: &Direction) -> bool {
        // attempt to move object into direction
        // if there is a box in the direction, attempt to move that first
        let next_pos = match *direction {
            Direction::Top => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0 + 1, pos.1),
        };
        match self.get(next_pos) {
            &GridPoint::Wall => return false,
            &GridPoint::Robot => {
                panic!("encountered robot at unexpected pos")
            }
            &GridPoint::Empty => {
                // move and return true
                self.grid[next_pos.0][next_pos.1] = self.get(pos).clone();
                self.grid[pos.0][pos.1] = GridPoint::Empty;
                true
            }
            &GridPoint::Box => {
                // attempt to move the box first, if it could move, try to move the current thing
                // again
                if self.move_to(next_pos, direction) {
                    return self.move_to(pos, direction);
                } else {
                    return false;
                }
            }
        }
    }

    fn get(&self, pos: (usize, usize)) -> &GridPoint {
        &self.grid[pos.0][pos.1]
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let mut grid: Vec<Vec<GridPoint>> = vec![];
    let mut instructions: Vec<Direction> = vec![];
    // loop to read grid
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut row: Vec<GridPoint> = vec![];
        for c in line.chars() {
            row.push(c.to_string().parse().unwrap());
        }
        grid.push(row);
    }

    // loop to read instructions
    for line in lines {
        for c in line.chars() {
            instructions.push(c.to_string().parse().unwrap());
        }
    }

    let mut warehouse = Warehouse::new(grid);

    for dir in instructions {
        warehouse.move_robot(dir);
    }

    println!("{}", warehouse.sum_box_coordinates());
}
