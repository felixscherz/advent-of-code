use std::{
    io::{stdin, Read},
    str::FromStr,
};

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
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":").nth(1).unwrap().split(",");
        let x: i32 = parts.next().unwrap().replace(" X+", "").parse().unwrap();
        let y: i32 = parts.next().unwrap().replace(" Y+", "").parse().unwrap();
        Ok(Self::new(x, y))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":").nth(1).unwrap().split(",");
        let x: i64 = parts.next().unwrap().replace(" X=", "").parse().unwrap();
        let y: i64 = parts.next().unwrap().replace(" Y=", "").parse().unwrap();
        let increase_by = 10000000000000i64;
        // let increase_by = 0i64;
        Ok(Self { x: x + increase_by, y: y + increase_by })
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn reachable_from(&self, other: &Point, moves: (&Move, &Move)) -> bool {
        // calculate the linear combination and check whether components are integers
        let target = Self::new(self.x - other.x, self.y - other.y);
        let move_a = &moves.0;
        let move_b = &moves.1;
        let mut b: f32 = (target.y as f32 - target.x as f32 * move_a.y as f32 / move_a.x as f32)
            / (move_b.y as f32 - move_a.y as f32 * move_b.x as f32 / move_a.x as f32);
        if b.is_nan() {
            b = 0f32;
        }
        let a: f32 = (target.x as f32 - b * move_b.x as f32) / move_a.x as f32;
        (a as f32).fract() == 0f32 && (b as f32).fract() == 0f32 && a >= 0f32 && b >= 0f32
    }

    fn apply(&self, step: &Move) -> Self {
        Self::new(self.x + step.x as i64, self.y + step.y as i64)
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

    fn cost(&self) -> Option<u64> {
        if !(self.a.x as f64 / self.b.x as f64 == self.a.y as f64 / self.b.y as f64) {
            // not degenerate
            let a_coeff = (self.prize.y as f64
                - self.b.y as f64 * self.prize.x as f64 / self.b.x as f64)
                / (self.a.y as f64 - self.b.y as f64 * self.a.x as f64 / self.b.x as f64);

            let b_coeff = (self.prize.x as f64 - a_coeff * self.a.x as f64) / self.b.x as f64;

            if !(is_integer(a_coeff) && is_integer(b_coeff) && a_coeff >= 0f64 && b_coeff >= 0f64) {
                return None;
            }



            return Some(3 * (a_coeff.round() as u64) + b_coeff.round() as u64);
        }

        // degenerate
        let k = 1f64 - 3f64 * self.a.x as f64 / self.b.x as f64;
        // if k == 0 then it doesn't matter which

        let r = self.b.x as f64 / self.a.x as f64;
        dbg!(r);

        match k {
            _ if k > 0f64 => {
                // cost grows with increasing b -> start with b=0 and increase
                for b in 0..100 {
                    let a_coeff = self.prize.x as f64 / self.a.x as f64 - b as f64 * r;
                    if !is_integer(a_coeff) {
                        continue;
                    }
                    return Some(3 * a_coeff as u64 + b as u64);
                }
            }
            _ if k < 0f64 => {
                // cost decreases with increasing b -> start with a=0 and increase,
                for a in 0..100 {
                    let b_coeff = (self.prize.x as f64 / self.a.x as f64 - a as f64) / r;
                    if !is_integer(b_coeff) {
                        continue;
                    }
                    dbg!(b_coeff);
                    dbg!(a);
                    return Some(b_coeff as u64 + 3 * a as u64);
                }
            }
            _ => {
                panic!("wowww");
            }
        }
        None
    }
}

fn is_integer(x: f64) -> bool {
    (x.round() - x).abs() < 0.00000000001f64
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let mut machines: Vec<Machine> = vec![];

    loop {
        let line = lines.next().unwrap();
        let move_a: Move = line.parse().unwrap();
        let move_b: Move = lines.next().unwrap().parse().unwrap();
        let prize: Point = lines.next().unwrap().parse().unwrap();

        machines.push(Machine {
            a: move_a,
            b: move_b,
            prize,
        });

        if lines.next().is_none() {
            break;
        }
    }

    let cost: u64 = machines.iter().filter_map(Machine::cost).sum();
    println!("{}", cost);
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

    #[test]
    fn test_machine_calculate_cost() {
        let machine = Machine {
            a: Move::new(1, 0),
            b: Move::new(0, 1),
            prize: Point::new(5, 5),
        };
        assert!(machine.winnable());
    }

    #[test]
    fn test_machine_takes_cheapest_path() {
        let machine = Machine {
            a: Move::new(4, 2),
            b: Move::new(2, 1),
            prize: Point::new(8, 4),
        };
        assert_eq!(machine.cost(), Some(4));
    }

    #[test]
    fn test_first_example() {
        let machine = Machine {
            a: Move::new(94, 34),
            b: Move::new(22, 67),
            prize: Point::new(8400, 5400),
        };
        assert_eq!(machine.cost(), Some(280));
    }

    #[test]
    fn test_broken_example_part2() {
        let machine = Machine {
            a: Move::new(49, 18),
            b: Move::new(40, 58),
            prize: Point::new(965, 1134),
        };
        assert_eq!(machine.cost(), Some(33));
    }
    #[test]
    fn test_is_integer() {
        assert!(is_integer(4.9999995f64));
    }
}
