use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbour_in_direction(self, direction: &Direction) -> Self {
        Self {
            x: match direction {
                Direction::Left => self.x - 1,
                Direction::Right => self.x + 1,
                _ => self.x,
            },
            y: match direction {
                Direction::Up => self.y + 1,
                Direction::Down => self.y - 1,
                _ => self.y,
            },
        }
    }

    fn follow(self, other: Self) -> Self {
        let candidate = Self {
            x: self.x + (other.x - self.x).signum(),
            y: self.y + (other.y - self.y).signum(),
        };
        if candidate.x == other.x && candidate.y == other.y {
            self
        } else {
            candidate
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseInstructionError;

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    steps: u32,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err(ParseInstructionError);
        };

        match parts[1].parse::<u32>() {
            Err(_) => Err(ParseInstructionError),
            Ok(steps) => {
                let direction = match parts.first() {
                    Some(&"U") => Ok(Direction::Up),
                    Some(&"D") => Ok(Direction::Down),
                    Some(&"L") => Ok(Direction::Left),
                    Some(&"R") => Ok(Direction::Right),
                    _ => Err(ParseInstructionError),
                }?;
                Ok(Instruction { direction, steps })
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(len: usize) -> Self {
        let mut knots = Vec::new();

        for _ in 0..len {
            knots.push(Point { x: 0, y: 0 });
        }

        Rope { knots }
    }

    fn execute_step(&self, direction: &Direction) -> Self {
        let mut knots = Vec::new();
        let mut prev = self.knots[0].neighbour_in_direction(direction);
        knots.push(prev);

        for ix in 1..self.knots.len() {
            let knot = self.knots[ix].follow(prev);
            knots.push(knot);
            prev = knot;
        }

        Rope { knots }
    }

    fn tail(&self) -> Point {
        *self.knots.last().unwrap_or(&Point { x: 0, y: 0 })
    }
}

fn tail_visits(input: &str, knots: usize) -> usize {
    let mut rope = Rope::new(knots);
    let mut visited: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        match line.parse::<Instruction>() {
            Err(_) => {}
            Ok(instruction) => {
                for _ in 0..instruction.steps {
                    rope = rope.execute_step(&instruction.direction);
                    visited.insert(rope.tail());
                }
            }
        };
    }

    visited.len()
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Some(tail_visits(input, 2))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Some(tail_visits(input, 10))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_follow() {
        assert_eq!(
            Point { x: 1, y: 1 }.follow(Point { x: 1, y: 3 }),
            Point { x: 1, y: 2 }
        );
    }

    #[test]
    fn test_point_follow_but_dont_overlap() {
        assert_eq!(
            Point { x: 1, y: 1 }.follow(Point { x: 1, y: 2 }),
            Point { x: 1, y: 1 }
        );
    }

    #[test]
    fn test_point_neighbour_in_direction() {
        assert_eq!(
            Point { x: 2, y: 3 }.neighbour_in_direction(&Direction::Up),
            Point { x: 2, y: 4 }
        );
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            "R 4".parse(),
            Ok(Instruction {
                direction: Direction::Right,
                steps: 4
            }),
        )
    }

    #[test]
    fn test_new_rope() {
        let rope = Rope::new(2);
        assert_eq!(rope.knots, vec![Point { x: 0, y: 0 }, Point { x: 0, y: 0 }]);
    }

    #[test]
    fn test_step_rope_tail_doesnt_overlap() {
        let before = Rope {
            knots: vec![Point { x: 0, y: 0 }, Point { x: 0, y: 0 }],
        };
        assert_eq!(
            before.execute_step(&Direction::Right).tail(),
            Point { x: 0, y: 0 }
        );
    }

    #[test]
    fn test_step_rope_tail_follows() {
        let before = Rope {
            knots: vec![Point { x: 1, y: 2 }, Point { x: 1, y: 1 }],
        };
        assert_eq!(
            before.execute_step(&Direction::Up).tail(),
            Point { x: 1, y: 2 }
        );
    }

    #[test]
    fn test_step_rope_tail_follows_diagonally() {
        let before = Rope {
            knots: vec![Point { x: 2, y: 2 }, Point { x: 1, y: 1 }],
        };
        assert_eq!(
            before.execute_step(&Direction::Right).tail(),
            Point { x: 2, y: 2 }
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
