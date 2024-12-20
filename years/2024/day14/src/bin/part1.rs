use std::{
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct BoundingBox {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl BoundingBox {
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}
struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position<'a> {
    x: i32,
    y: i32,
    bbox: &'a BoundingBox,
}

impl<'a> Position<'a> {
    fn new(x: i32, y: i32, bbox: &'a BoundingBox) -> Self {
        Self { x, y, bbox }
    }
    fn move_by(&self, x: i32, y: i32) -> Self {
        let width = self.bbox.x_max - self.bbox.x_min + 1;
        let height = self.bbox.y_max - self.bbox.y_min + 1;
        let x_new = match (self.x + x) % width {
            x if x < 0 => width + x,
            x => x,
        };

        let y_new = match (self.y + y) % height {
            y if y < 0 => height + y,
            y => y,
        };
        Self::new(x_new, y_new, self.bbox)
    }

    fn quadrant(&self) -> Option<Quadrant> {
        match (self.x, self.y) {
            (x, y) if x < self.bbox.x_max / 2 && y < self.bbox.y_max / 2 => Some(Quadrant::TopLeft),
            (x, y) if x > self.bbox.x_max / 2 && y < self.bbox.y_max / 2 => {
                Some(Quadrant::TopRight)
            }
            (x, y) if x < self.bbox.x_max / 2 && y > self.bbox.y_max / 2 => {
                Some(Quadrant::BottomLeft)
            }
            (x, y) if x > self.bbox.x_max / 2 && y > self.bbox.y_max / 2 => {
                Some(Quadrant::BottomRight)
            }
            _ => None,
        }
    }
}

struct Robot<'a> {
    pos: Position<'a>,
    v: Velocity,
}

impl<'a> Robot<'a> {
    fn step(&mut self) {
        self.pos = self.pos.move_by(self.v.x, self.v.y);
    }
    fn new(pos: Position<'a>, v: Velocity) -> Self {
        Self { pos, v }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let bbox = BoundingBox::new(0, 100, 0, 102);
    // let bbox = BoundingBox::new(0, 10, 0, 6);
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let mut parts = line.split(" ");
        let pos_parts = parts.next().unwrap().replace("p=", "");
        let pos_x: i32 = pos_parts.split(",").nth(0).unwrap().parse().unwrap();
        let pos_y: i32 = pos_parts.split(",").nth(1).unwrap().parse().unwrap();
        let vel_parts = parts.next().unwrap().replace("v=", "");
        let v_x: i32 = vel_parts.split(",").nth(0).unwrap().parse().unwrap();
        let v_y: i32 = vel_parts.split(",").nth(1).unwrap().parse().unwrap();
        let pos = Position::new(pos_x, pos_y, &bbox);
        let v = Velocity::new(v_x, v_y);
        robots.push(Robot::new(pos, v));
    }

    for i in 0..100 {
        for j in 0..robots.len() {
            robots[j].step();
        }
    }

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    for robot in robots {
        match robot.pos.quadrant() {
            None => (),
            Some(Quadrant::TopLeft) => top_left = top_left + 1,
            Some(Quadrant::TopRight) => top_right = top_right + 1,
            Some(Quadrant::BottomLeft) => bottom_left = bottom_left + 1,
            Some(Quadrant::BottomRight) => bottom_right = bottom_right + 1,
        }
    }
    let total = top_left * top_right * bottom_left * bottom_right;
    println!("{}", total);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wrapping_position() {
        let bbox = BoundingBox::new(0, 10, 0, 10);
        let pos = Position::new(0, 0, &bbox);
        assert_eq!(pos.move_by(10, 10), Position::new(10, 10, &bbox));
    }
    #[test]
    fn test_remainder() {
        assert_eq!(5 % 10, 5);
        assert_eq!(-1 % 10, -1);
    }
}
