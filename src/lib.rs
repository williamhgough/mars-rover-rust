use std::{fmt::Error, str::FromStr};

#[derive(Debug)]
pub struct Rover {
    boundaries: (i32, i32),
    coordinates: (i32, i32),
    direction: Direction,
}

impl Rover {
    pub fn new(boundaries: &str) -> Self {
        let bds = Rover::parse_boundaries(boundaries);
        Self {
            coordinates: (0, 0),
            direction: Direction::North,
            boundaries: (bds.0, bds.1),
        }
    }

    fn parse_boundaries(input: &str) -> (i32, i32) {
        let mut ins = input.split_whitespace();
        let x = ins.next().unwrap().parse().unwrap();
        let y = ins.next().unwrap().parse().unwrap();
        (x, y)
    }

    pub fn set_position(&mut self, input: &str) {
        let mut ins = input.split_whitespace();
        self.coordinates.0 = ins.next().unwrap().parse().unwrap();
        self.coordinates.1 = ins.next().unwrap().parse().unwrap();
        self.direction = ins.next().unwrap().parse().unwrap();
    }

    pub fn process_input(&mut self, input: &str) {
        input.chars().for_each(|c| match c {
            'L' | 'R' => self.change(&c),
            'M' => match self.direction {
                Direction::North if self.can_move_up() => self.coordinates.1 += 1,
                Direction::East if self.can_move_right() => self.coordinates.0 += 1,
                Direction::South if self.can_move_down() => self.coordinates.1 -= 1,
                Direction::West if self.can_move_left() => self.coordinates.0 -= 1,
                _ => {
                    println!("oops: {:?}, input: {:?}", &self, input);
                }
            },
            _ => {}
        });
    }

    pub fn get_position(&self) -> String {
        format!(
            "{} {} {}",
            self.coordinates.0,
            self.coordinates.1,
            self.direction.to_string()
        )
    }

    pub fn get_boundaries(&self) -> String {
        format!("{} {}", self.boundaries.0, self.boundaries.1,)
    }

    fn change(&mut self, turn: &char) {
        match self.direction {
            Direction::North => match turn {
                'L' => self.direction = Direction::West,
                'R' => self.direction = Direction::East,
                _ => {}
            },
            Direction::East => match turn {
                'L' => self.direction = Direction::North,
                'R' => self.direction = Direction::South,
                _ => {}
            },
            Direction::South => match turn {
                'L' => self.direction = Direction::East,
                'R' => self.direction = Direction::West,
                _ => {}
            },
            Direction::West => match turn {
                'L' => self.direction = Direction::South,
                'R' => self.direction = Direction::North,
                _ => {}
            },
        }
    }

    fn can_move_up(&self) -> bool {
        (self.coordinates.1 + 1) <= self.boundaries.1
    }

    fn can_move_down(&self) -> bool {
        (self.coordinates.1 - 1) >= 0
    }

    fn can_move_left(&self) -> bool {
        (self.coordinates.0 - 1) >= 0
    }

    fn can_move_right(&self) -> bool {
        (self.coordinates.0 + 1) <= self.boundaries.0
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::North => String::from("N"),
            Direction::East => String::from("E"),
            Direction::South => String::from("S"),
            Direction::West => String::from("W"),
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Direction::North),
            "E" => Ok(Direction::East),
            "S" => Ok(Direction::South),
            "W" => Ok(Direction::West),
            _ => Err(Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_boundaries() {
        let rover1 = Rover::new("5 5");
        assert_eq!(rover1.get_boundaries(), "5 5");
    }

    #[test]
    fn sets_position() {
        let mut rover1 = Rover::new("5 5");
        rover1.set_position("1 2 N");
        assert_eq!(rover1.get_position(), "1 2 N");
    }

    #[test]
    fn moves_left() {
        let mut rover1 = Rover::new("5 5");
        rover1.set_position("1 2 N");
        rover1.process_input("LM");

        assert_eq!(rover1.get_position(), "0 2 W");
    }

    #[test]
    fn moves_right() {
        let mut rover1 = Rover::new("5 5");
        rover1.set_position("1 2 N");
        rover1.process_input("RM");

        assert_eq!(rover1.get_position(), "2 2 E");
    }

    #[test]
    fn moves_forward() {
        let mut rover1 = Rover::new("5 5");
        rover1.set_position("1 2 E");
        rover1.process_input("MMM");

        assert_eq!(rover1.get_position(), "4 2 E");
    }

    #[test]
    fn rover_one() {
        let mut rover1 = Rover::new("5 5");
        rover1.set_position("1 2 N");
        rover1.process_input("LMLMLMLMM");
        assert_eq!(rover1.get_position(), "1 3 N");
    }

    #[test]
    fn rover_two() {
        let mut rover1 = Rover::new("5 5");
        rover1.set_position("3 3 E");
        rover1.process_input("MMRMMRMRRM");
        assert_eq!(rover1.get_position(), "5 1 E");
    }
}
