use std::io::{stdin, Read};

#[derive(Debug)]
struct Move {
    x: i32,
    y: i32,
}

impl Move {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn reachable_from(&self, other: &Point, moves: (&Move, &Move)) -> bool {
        // calculate the linear combination and check whether components are integers
        let target = Self::new(self.x - other.x, self.y - other.y);
        let move_a = &moves.0;
        let move_b = &moves.1;
        let b: f32 = (target.y as f32 - target.x as f32 * move_a.y as f32 / move_a.x as f32)
            / (move_b.y as f32 - move_a.y as f32 * move_b.x as f32 / move_a.x as f32);
        let a: f32 = (target.x as f32 - b * move_b.x as f32) / move_a.x as f32;
        dbg!(a);
        dbg!(b);
        (a as f32).fract() == 0f32 && (b as f32).fract() == 0f32 && a >= 0f32 && b >= 0f32
    }
}

#[derive(Debug)]
struct Machine {
    a: Move,
    b: Move,
    prize: Point,
}

impl Machine {
    fn winnable(&self) -> bool {
        let start = Point::new(0, 0);
        self.prize.reachable_from(&start, (&self.a, &self.b))
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // algorithm will be to add nodes all around the starting location at every point reachable
    // until that point cloud contains the prize
    // from there use djikstars algorithm to find the shortest path to the node
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reachable() {
        let start = Point::new(1, 1);
        let target = Point::new(5, 5);
        let moves = (&Move::new(1, 0), &Move::new(0, 1));
        assert!(target.reachable_from(&start, moves));
    }

    #[test]
    fn test_not_reachable_due_to_fraction() {
        let start = Point::new(0, 0);
        let target = Point::new(5, 5);
        let moves = (&Move::new(2, 0), &Move::new(0, 2));
        assert!(!target.reachable_from(&start, moves));
    }

    #[test]
    fn test_not_reachable_due_to_wrong_direction() {
        let start = Point::new(0, 0);
        let target = Point::new(-4, 4);
        let moves = (&Move::new(2, 0), &Move::new(0, 2));
        assert!(!target.reachable_from(&start, moves));
    }
}
