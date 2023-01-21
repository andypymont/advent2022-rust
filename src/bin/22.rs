use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Open,
    Wall,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => Tile::Empty,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Forward(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, PartialEq)]
struct InstructionCollector {
    collected: Vec<Instruction>,
    current: String,
}

impl InstructionCollector {
    fn new() -> Self {
        InstructionCollector {
            collected: Vec::new(),
            current: String::new(),
        }
    }

    fn push_current(&mut self) {
        if !self.current.is_empty() {
            if let Ok(steps) = self.current.parse::<u32>() {
                self.collected.push(Instruction::Forward(steps));
                self.current.clear();
            }
        }
    }

    fn push_char(&mut self, c: char) {
        match c {
            'L' => {
                self.push_current();
                self.collected.push(Instruction::TurnLeft);
            }
            'R' => {
                self.push_current();
                self.collected.push(Instruction::TurnRight);
            }
            _ => {
                if c.is_numeric() {
                    self.current.push(c);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    facing: Direction,
}

impl Position {
    fn password(&self) -> u32 {
        let row: u32 = self.y.try_into().unwrap_or(0) + 1;
        let col: u32 = self.x.try_into().unwrap_or(0) + 1;
        let facing = match self.facing {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        (1000 * row) + (4 * col) + facing
    }

    fn execute_instruction(&self, map: &GroveMap, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::TurnLeft => Self {
                x: self.x,
                y: self.y,
                facing: self.facing.turn_left(),
            },
            Instruction::TurnRight => Self {
                x: self.x,
                y: self.y,
                facing: self.facing.turn_right(),
            },
            Instruction::Forward(steps) => {
                let mut x = self.x;
                let mut y = self.y;

                for _ in 0..*steps {
                    let min_x = map.first_col_in_row(y);
                    let max_x = map.last_col_in_row(y);
                    let min_y = map.first_row_in_col(x);
                    let max_y = map.last_row_in_col(x);

                    let try_x = match self.facing {
                        Direction::Left => {
                            if x <= min_x {
                                max_x
                            } else {
                                x - 1
                            }
                        }
                        Direction::Right => {
                            if x >= max_x {
                                min_x
                            } else {
                                x + 1
                            }
                        }
                        _ => x,
                    };
                    let try_y = match self.facing {
                        Direction::Up => {
                            if y <= min_y {
                                max_y
                            } else {
                                y - 1
                            }
                        }
                        Direction::Down => {
                            if y >= max_y {
                                min_y
                            } else {
                                y + 1
                            }
                        }
                        _ => y,
                    };
                    (x, y) = match map.get_tile(try_x, try_y) {
                        Tile::Open => (try_x, try_y),
                        Tile::Wall => (x, y),
                        Tile::Empty => match self.facing {
                            Direction::Up => (x, max_y),
                            Direction::Down => (x, min_y),
                            Direction::Left => (max_x, y),
                            Direction::Right => (min_x, y),
                        },
                    };
                }

                Position {
                    x,
                    y,
                    facing: self.facing.clone(),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseInputError;

#[derive(Debug, PartialEq)]
struct GroveMap {
    height: usize,
    width: usize,
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for GroveMap {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().map(str::len).max().unwrap_or(0);
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| {
                let mut line: Vec<Tile> = line.chars().map(Tile::from_char).collect();
                let extra = 0.max(width - line.len());
                for _ in 0..extra {
                    line.push(Tile::Empty);
                }
                line
            })
            .collect();
        if tiles.is_empty() || width == 0 {
            Err(ParseInputError)
        } else {
            Ok(Self {
                height: tiles.len(),
                width,
                tiles,
            })
        }
    }
}

impl GroveMap {
    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        match self.tiles.get(y) {
            Some(row) => row.get(x).unwrap_or(&Tile::Empty),
            None => &Tile::Empty,
        }
    }

    fn first_col_in_row(&self, y: usize) -> usize {
        match self.tiles.get(y) {
            Some(row) => row.iter().position(|t| t != &Tile::Empty).unwrap_or(0),
            None => 0,
        }
    }

    fn last_col_in_row(&self, y: usize) -> usize {
        match self.tiles.get(y) {
            Some(row) => row.iter().rposition(|t| t != &Tile::Empty).unwrap_or(0),
            None => self.width - 1,
        }
    }

    fn first_row_in_col(&self, x: usize) -> usize {
        self.tiles
            .iter()
            .map(|row| row.get(x).unwrap_or(&Tile::Empty))
            .position(|t| t != &Tile::Empty)
            .unwrap_or(0)
    }

    fn last_row_in_col(&self, x: usize) -> usize {
        self.tiles
            .iter()
            .map(|row| row.get(x).unwrap_or(&Tile::Empty))
            .rposition(|t| t != &Tile::Empty)
            .unwrap_or(self.height - 1)
    }

    fn create_initial_position(&self) -> Position {
        Position {
            x: self.first_col_in_row(0),
            y: 0,
            facing: Direction::Right,
        }
    }

    fn follow_instructions(&self, instructions: &Vec<Instruction>) -> Position {
        let mut position = self.create_initial_position();

        for instruction in instructions {
            position = position.execute_instruction(self, instruction);
        }

        position
    }
}

fn parse_input(input: &str) -> Result<(GroveMap, Vec<Instruction>), ParseInputError> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() == 2 {
        let map: GroveMap = parts[0].parse()?;

        let mut collector = InstructionCollector::new();
        for c in parts[1].chars() {
            collector.push_char(c);
        }
        collector.push_current();

        Ok((map, collector.collected))
    } else {
        Err(ParseInputError)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok((map, instructions)) = parse_input(input) {
        Some(map.follow_instructions(&instructions).password())
    } else {
        None
    }
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 22);
        if let Ok((map, instructions)) = parse_input(&input) {
            assert_eq!(map.height, 12);
            assert_eq!(map.width, 16);

            assert_eq!(map.get_tile(0, 0), &Tile::Empty);
            assert_eq!(map.get_tile(8, 1), &Tile::Open);
            assert_eq!(map.get_tile(9, 1), &Tile::Wall);
            assert_eq!(map.get_tile(0, 4), &Tile::Open);
            assert_eq!(map.get_tile(3, 4), &Tile::Wall);
            assert_eq!(map.get_tile(4, 4), &Tile::Open);
            assert_eq!(
                map.create_initial_position(),
                Position {
                    x: 8,
                    y: 0,
                    facing: Direction::Right,
                },
            );

            assert_eq!(map.first_col_in_row(0), 8);
            assert_eq!(map.last_col_in_row(0), 11);
            assert_eq!(map.first_row_in_col(1), 4);
            assert_eq!(map.last_row_in_col(1), 7);

            assert_eq!(
                instructions,
                vec![
                    Instruction::Forward(10),
                    Instruction::TurnRight,
                    Instruction::Forward(5),
                    Instruction::TurnLeft,
                    Instruction::Forward(5),
                    Instruction::TurnRight,
                    Instruction::Forward(10),
                    Instruction::TurnLeft,
                    Instruction::Forward(4),
                    Instruction::TurnRight,
                    Instruction::Forward(5),
                    Instruction::TurnLeft,
                    Instruction::Forward(5),
                ]
            );
        }
    }

    #[test]
    fn test_password() {
        let position = Position {
            x: 7,
            y: 5,
            facing: Direction::Right,
        };
        assert_eq!(position.password(), 6032,)
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
