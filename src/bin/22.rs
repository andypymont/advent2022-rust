use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ParseInputError;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(usize, usize);

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        let y = match direction {
            Direction::Up => self.1 - 1,
            Direction::Down => self.1 + 1,
            _ => self.1,
        };
        let x = match direction {
            Direction::Left => self.0 - 1,
            Direction::Right => self.0 + 1,
            _ => self.0,
        };
        Self(x, y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct Square {
    position: Position,
    tiles: Vec<Vec<Tile>>,
}

impl Square {
    fn first_open_position(&self) -> Position {
        let x = {
            if let Some(row) = self.tiles.first() {
                row.iter().position(|tile| tile == &Tile::Open).unwrap_or(0)
            } else {
                0
            }
        };
        Position(x, 0)
    }

    fn is_position_open(&self, position: Position) -> bool {
        if let Some(row) = self.tiles.get(position.1) {
            matches!(row.get(position.0), Some(Tile::Open))
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

const COMPASS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Right,
    Direction::Down,
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Edge {
    square: usize,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CubePosition {
    square: usize,
    position: Position,
    facing: Direction,
}

impl CubePosition {
    fn to_edge(self) -> Edge {
        Edge {
            square: self.square,
            direction: self.facing,
        }
    }

    fn to_flat_position(self, map: &GroveMap) -> Position {
        if let Some(square) = map.squares.get(self.square) {
            Position(
                (square.position.0 * map.square_size) + self.position.0,
                (square.position.1 * map.square_size) + self.position.1,
            )
        } else {
            self.position
        }
    }

    fn position_ahead(self, map: &GroveMap) -> Self {
        let max_coord = map.square_size - 1;
        if (self.position.0 == 0 && self.facing == Direction::Left)
            || (self.position.1 == 0 && self.facing == Direction::Up)
            || (self.position.0 == max_coord && self.facing == Direction::Right)
            || (self.position.1 == max_coord && self.facing == Direction::Down)
        {
            self.traverse_edge(map)
        } else {
            Self {
                position: self.position + self.facing,
                ..self
            }
        }
    }

    fn traverse_edge(&self, map: &GroveMap) -> Self {
        let pos = match self.facing {
            Direction::Up => self.position.0,
            Direction::Right => self.position.1,
            Direction::Down => map.square_size - self.position.0,
            Direction::Left => map.square_size - self.position.1,
        };

        if let Some(enter_edge) = map.connections.get(&self.to_edge()) {
            let facing = enter_edge.direction.reverse();
            let x = match facing {
                Direction::Up => pos,
                Direction::Down => map.square_size - pos,
                Direction::Left => map.square_size - 1,
                Direction::Right => 0,
            };
            let y = match facing {
                Direction::Right => pos,
                Direction::Left => map.square_size - pos,
                Direction::Up => map.square_size - 1,
                Direction::Down => 0,
            };
            Self {
                square: enter_edge.square,
                facing,
                position: Position(x, y),
            }
        } else {
            *self
        }
    }

    fn password(&self, map: &GroveMap) -> u32 {
        let position = self.to_flat_position(map);
        let col = u32::try_from(position.0).unwrap_or(0) + 1;
        let row = u32::try_from(position.1).unwrap_or(0) + 1;

        let facing = match self.facing {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        (1000 * row) + (4 * col) + facing
    }
}

#[derive(Debug, PartialEq)]
struct GroveLayout {
    layout: Vec<Position>,
}

impl GroveLayout {
    fn new() -> Self {
        Self { layout: Vec::new() }
    }

    fn insert(&mut self, position: Position) {
        self.layout.push(position);
    }

    fn first_square_in_column(&self, x: usize) -> usize {
        self.layout.iter().position(|pos| pos.0 == x).unwrap_or(0)
    }

    fn last_square_in_column(&self, x: usize) -> usize {
        self.layout.iter().rposition(|pos| pos.0 == x).unwrap_or(0)
    }

    fn first_square_in_row(&self, y: usize) -> usize {
        self.layout.iter().position(|pos| pos.1 == y).unwrap_or(0)
    }

    fn last_square_in_row(&self, y: usize) -> usize {
        self.layout.iter().rposition(|pos| pos.1 == y).unwrap_or(0)
    }

    fn square_in_direction(&self, position: Position, direction: Direction) -> usize {
        let default = match direction {
            Direction::Up => self.last_square_in_column(position.0),
            Direction::Right => self.first_square_in_row(position.1),
            Direction::Down => self.first_square_in_column(position.0),
            Direction::Left => self.last_square_in_row(position.1),
        };

        if (direction == Direction::Up && position.1 == 0)
            || (direction == Direction::Left && position.0 == 0)
        {
            default
        } else if let Some(square) = self
            .layout
            .iter()
            .position(|pos| pos == &(position + direction))
        {
            square
        } else {
            default
        }
    }

    fn get_connections(&self) -> HashMap<Edge, Edge> {
        let mut connections = HashMap::new();

        for (number, position) in self.layout.iter().enumerate() {
            for direction in COMPASS {
                let edge = Edge {
                    square: number,
                    direction,
                };
                let other = self.square_in_direction(*position, direction);
                let other_edge = Edge {
                    square: other,
                    direction: direction.reverse(),
                };
                connections.insert(edge, other_edge);
            }
        }

        connections
    }
}

#[derive(Debug, PartialEq)]
struct GroveMap {
    square_size: usize,
    squares: Vec<Square>,
    connections: HashMap<Edge, Edge>,
}

impl FromStr for GroveMap {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (square_size, longest_line, tiles): (usize, usize, Vec<Vec<Tile>>) = s.lines().fold(
            (usize::MAX, 0, Vec::new()),
            |(square_size, longest_line, mut tiles), line| {
                tiles.push(line.chars().map(Tile::from_char).collect());
                (
                    square_size.min(line.trim().len()),
                    longest_line.max(line.len()),
                    tiles,
                )
            },
        );

        let mut squares = Vec::new();
        let mut layout = GroveLayout::new();

        for top in (0..tiles.len()).step_by(square_size) {
            if let Some(row) = tiles.get(top) {
                for left in (0..longest_line).step_by(square_size) {
                    if let Some(top_left_tile_type) = row.get(left) {
                        if top_left_tile_type == &Tile::Empty {
                            continue;
                        }

                        let position = Position(left / square_size, top / square_size);

                        let square_tiles: Vec<Vec<Tile>> = (top..top + square_size)
                            .filter_map(|y| {
                                tiles.get(y).map(|row| {
                                    (left..left + square_size)
                                        .map(|x| *row.get(x).unwrap_or(&Tile::Empty))
                                        .collect()
                                })
                            })
                            .collect();

                        squares.push(Square {
                            position,
                            tiles: square_tiles,
                        });
                        layout.insert(position);
                    }
                }
            }
        }

        Ok(GroveMap {
            square_size,
            squares,
            connections: layout.get_connections(),
        })
    }
}

impl GroveMap {
    fn create_initial_position(&self) -> CubePosition {
        let position = {
            if let Some(square) = self.squares.first() {
                square.first_open_position()
            } else {
                Position(0, 0)
            }
        };
        CubePosition {
            square: 0,
            position,
            facing: Direction::Right,
        }
    }

    fn position_after_instruction(
        &self,
        cube_pos: CubePosition,
        instruction: &Instruction,
    ) -> CubePosition {
        match instruction {
            Instruction::TurnLeft => CubePosition {
                facing: cube_pos.facing.turn_left(),
                ..cube_pos
            },
            Instruction::TurnRight => CubePosition {
                facing: cube_pos.facing.turn_right(),
                ..cube_pos
            },
            Instruction::Forward(steps) => {
                let mut cube_pos = cube_pos;
                for _ in 0..*steps {
                    let ahead = cube_pos.position_ahead(self);
                    if let Some(square) = self.squares.get(ahead.square) {
                        if square.is_position_open(ahead.position) {
                            cube_pos = ahead;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                cube_pos
            }
        }
    }

    fn follow_instructions(&self, instructions: &Vec<Instruction>) -> CubePosition {
        let mut position = self.create_initial_position();

        for instruction in instructions {
            position = self.position_after_instruction(position, instruction);
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
        Some(map.follow_instructions(&instructions).password(&map))
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

    fn example_grove_map() -> GroveMap {
        let connections = {
            let zero_top = Edge {
                square: 0,
                direction: Direction::Up,
            };
            let zero_right = Edge {
                square: 0,
                direction: Direction::Right,
            };
            let zero_bottom = Edge {
                square: 0,
                direction: Direction::Down,
            };
            let zero_left = Edge {
                square: 0,
                direction: Direction::Left,
            };

            let one_top = Edge {
                square: 1,
                direction: Direction::Up,
            };
            let one_right = Edge {
                square: 1,
                direction: Direction::Right,
            };
            let one_bottom = Edge {
                square: 1,
                direction: Direction::Down,
            };
            let one_left = Edge {
                square: 1,
                direction: Direction::Left,
            };

            let two_top = Edge {
                square: 2,
                direction: Direction::Up,
            };
            let two_right = Edge {
                square: 2,
                direction: Direction::Right,
            };
            let two_bottom = Edge {
                square: 2,
                direction: Direction::Down,
            };
            let two_left = Edge {
                square: 2,
                direction: Direction::Left,
            };

            let three_top = Edge {
                square: 3,
                direction: Direction::Up,
            };
            let three_right = Edge {
                square: 3,
                direction: Direction::Right,
            };
            let three_bottom = Edge {
                square: 3,
                direction: Direction::Down,
            };
            let three_left = Edge {
                square: 3,
                direction: Direction::Left,
            };

            let four_top = Edge {
                square: 4,
                direction: Direction::Up,
            };
            let four_right = Edge {
                square: 4,
                direction: Direction::Right,
            };
            let four_bottom = Edge {
                square: 4,
                direction: Direction::Down,
            };
            let four_left = Edge {
                square: 4,
                direction: Direction::Left,
            };

            let five_top = Edge {
                square: 5,
                direction: Direction::Up,
            };
            let five_right = Edge {
                square: 5,
                direction: Direction::Right,
            };
            let five_bottom = Edge {
                square: 5,
                direction: Direction::Down,
            };
            let five_left = Edge {
                square: 5,
                direction: Direction::Left,
            };

            let mut connections = HashMap::new();

            connections.insert(zero_top, four_bottom);
            connections.insert(zero_right, zero_left);
            connections.insert(zero_bottom, three_top);
            connections.insert(zero_left, zero_right);

            connections.insert(one_top, one_bottom);
            connections.insert(one_right, two_left);
            connections.insert(one_bottom, one_top);
            connections.insert(one_left, three_right);

            connections.insert(two_top, two_bottom);
            connections.insert(two_right, three_left);
            connections.insert(two_bottom, two_top);
            connections.insert(two_left, one_right);

            connections.insert(three_top, zero_bottom);
            connections.insert(three_right, one_left);
            connections.insert(three_bottom, four_top);
            connections.insert(three_left, two_right);

            connections.insert(four_top, three_bottom);
            connections.insert(four_right, five_left);
            connections.insert(four_bottom, zero_top);
            connections.insert(four_left, five_right);

            connections.insert(five_top, five_bottom);
            connections.insert(five_right, four_left);
            connections.insert(five_bottom, five_top);
            connections.insert(five_left, four_right);

            connections
        };

        GroveMap {
            square_size: 4,
            squares: vec![
                Square {
                    position: Position(2, 0),
                    tiles: vec![
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Wall],
                        vec![Tile::Open, Tile::Wall, Tile::Open, Tile::Open],
                        vec![Tile::Wall, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                    ],
                },
                Square {
                    position: Position(0, 1),
                    tiles: vec![
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Wall],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Wall, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                    ],
                },
                Square {
                    position: Position(1, 1),
                    tiles: vec![
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Wall],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                    ],
                },
                Square {
                    position: Position(2, 1),
                    tiles: vec![
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Wall],
                        vec![Tile::Wall, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Wall, Tile::Open],
                    ],
                },
                Square {
                    position: Position(2, 2),
                    tiles: vec![
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Wall],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Wall, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                    ],
                },
                Square {
                    position: Position(3, 2),
                    tiles: vec![
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Wall, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Open, Tile::Open],
                        vec![Tile::Open, Tile::Open, Tile::Wall, Tile::Open],
                    ],
                },
            ],
            connections,
        }
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 22);
        let expected = Ok((
            example_grove_map(),
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
            ],
        ));
        assert_eq!(parse_input(&input), expected);
    }

    #[test]
    fn test_wrap_around() {
        let map = example_grove_map();
        let cube_pos = CubePosition {
            square: 0,
            position: Position(0, 0),
            facing: Direction::Up,
        };
        assert_eq!(
            cube_pos.position_ahead(&map),
            CubePosition {
                square: 4,
                position: Position(0, 3),
                facing: Direction::Up,
            }
        );
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
