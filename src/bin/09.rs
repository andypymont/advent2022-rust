use std::{collections::HashSet, ops::Add, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn toward(&self, other: &Self) -> Self {
        Self {
            x: (other.x - self.x).signum(),
            y: (other.y - self.y).signum(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Step {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
struct ParseInstructionError;

#[derive(Debug, PartialEq)]
struct Instruction {
    step: Step,
    times: u32,
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
            Ok(times) => {
                let step = match parts[0] {
                    "U" => Ok(Step::Up),
                    "D" => Ok(Step::Down),
                    "L" => Ok(Step::Left),
                    "R" => Ok(Step::Right),
                    _ => Err(ParseInstructionError),
                }?;
                Ok(Instruction { step, times })
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        }
    }

    fn make_step(&self, step: &Step) -> Self {
        let head = Point {
            x: self.head.x
                + match step {
                    Step::Left => -1,
                    Step::Right => 1,
                    _ => 0,
                },
            y: self.head.y
                + match step {
                    Step::Up => 1,
                    Step::Down => -1,
                    _ => 0,
                },
        };

        let move_tail = self.tail + self.tail.toward(&head);
        let tail = if move_tail == head {
            self.tail
        } else {
            move_tail
        };

        Rope { head, tail }
    }

    fn execute_instruction(self, instruction: Instruction) -> (Self, HashSet<Point>) {
        let mut visited = HashSet::new();
        let mut rope = self;
        for _ in 0..instruction.times {
            rope = rope.make_step(&instruction.step);
            visited.insert(rope.tail);
        }
        (rope, visited)
    }
}

fn tail_visits(input: &str, _tail_length: usize) -> usize {
    let mut rope = Rope::new();
    let mut visited: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        rope = match line.parse::<Instruction>() {
            Err(_) => rope,
            Ok(instruction) => {
                let (moved, newly_visited) = rope.execute_instruction(instruction);
                visited.extend(newly_visited);
                moved
            }
        };
    }

    visited.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(tail_visits(input, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
    fn test_step_head_from_start() {
        let before = Rope {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        };
        let after = Rope {
            head: Point { x: 1, y: 0 },
            tail: Point { x: 0, y: 0 },
        };
        assert_eq!(before.make_step(&Step::Right), after);
    }

    #[test]
    fn test_step_head_with_following_tail() {
        let before = Rope {
            head: Point { x: 1, y: 2 },
            tail: Point { x: 1, y: 1 },
        };
        let after = Rope {
            head: Point { x: 1, y: 3 },
            tail: Point { x: 1, y: 2 },
        };
        assert_eq!(before.make_step(&Step::Up), after);
    }

    #[test]
    fn test_step_head_diagonally_with_following_tail() {
        let before = Rope {
            head: Point { x: 2, y: 2 },
            tail: Point { x: 1, y: 1 },
        };
        let after = Rope {
            head: Point { x: 3, y: 2 },
            tail: Point { x: 2, y: 2 },
        };
        assert_eq!(before.make_step(&Step::Right), after);
    }

    #[test]
    fn test_parse_instruction() {
        let expected = Ok(Instruction {
            step: Step::Right,
            times: 4,
        });
        assert_eq!("R 4".parse(), expected);
    }

    #[test]
    fn test_execute_instruction() {
        let before = Rope {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        };
        let instruction = Instruction {
            step: Step::Right,
            times: 4,
        };
        let expected = Rope {
            head: Point { x: 4, y: 0 },
            tail: Point { x: 3, y: 0 },
        };

        let (after, visited) = before.execute_instruction(instruction);
        assert_eq!(after, expected);
        assert_eq!(visited.len(), 4);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        // assert_eq!(part_two(&input), Some(36));
        assert_eq!(part_two(&input), None);
    }
}
