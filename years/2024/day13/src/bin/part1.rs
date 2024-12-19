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
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":").nth(1).unwrap().split(",");
        let x: i32 = parts.next().unwrap().replace(" X=", "").parse().unwrap();
        let y: i32 = parts.next().unwrap().replace(" Y=", "").parse().unwrap();
        Ok(Self { x, y })
    }
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
        let mut b: f32 = (target.y as f32 - target.x as f32 * move_a.y as f32 / move_a.x as f32)
            / (move_b.y as f32 - move_a.y as f32 * move_b.x as f32 / move_a.x as f32);
        if b.is_nan() {
            b = 0f32;
        }
        let a: f32 = (target.x as f32 - b * move_b.x as f32) / move_a.x as f32;
        (a as f32).fract() == 0f32 && (b as f32).fract() == 0f32 && a >= 0f32 && b >= 0f32
    }

    fn apply(&self, step: &Move) -> Self {
        Self::new(self.x + step.x, self.y + step.y)
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

    fn cost(&self) -> Option<u32> {
        // if !self.winnable() {
        //     return None;
        // }

        let mut costs: Vec<u32> = vec![];
        let mut a_coeff: i32 = 0;
        loop {
            // if a_coeff is outside the cirlce (with b_coeff = 0), break;
            if a_coeff > 100 {
                break;
            }
            if a_coeff.pow(2) * (self.a.x.pow(2) + self.a.y.pow(2))
                > self.prize.x.pow(2) + self.prize.y.pow(2)
            {
                break;
            }
            let mut b_coeff: i32 = 0;
            loop {
                // if coeffs are out of the circle, break inner loop
                if b_coeff > 100 {
                    break;
                }
                if (a_coeff * self.a.x + b_coeff * self.b.x).pow(2)
                    + (a_coeff * self.a.y + b_coeff * self.b.y).pow(2)
                    > self.prize.x.pow(2) + self.prize.y.pow(2)
                {
                    break;
                }

                // if coefficients solve equation, push to costs
                if (a_coeff * self.a.x + b_coeff * self.b.x) == self.prize.x
                    && (a_coeff * self.a.y + b_coeff * self.b.y) == self.prize.y
                {
                    costs.push(3 * a_coeff as u32 + b_coeff as u32)
                }

                b_coeff = b_coeff + 1;
            }

            a_coeff = a_coeff + 1;
        }

        costs.iter().min().copied()
    }
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

        machines.push(Machine{a:move_a, b:move_b, prize});

        if lines.next().is_none() {
            break;
        }

    }

    dbg!(machines.len());
    dbg!(&machines.last().unwrap());

    let cost: u32 = machines.iter().filter_map(Machine::cost).sum();
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
}
