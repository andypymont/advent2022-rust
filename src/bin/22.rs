use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

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
        let max_dimension = map.square_size - 1;

        let pos = match self.facing {
            Direction::Up => self.position.0,
            Direction::Right => self.position.1,
            Direction::Down => max_dimension - self.position.0,
            Direction::Left => max_dimension - self.position.1,
        };

        if let Some(enter_edge) = map.connections.get(&self.to_edge()) {
            let x = match enter_edge.direction {
                Direction::Up => max_dimension - pos,
                Direction::Right => max_dimension,
                Direction::Down => pos,
                Direction::Left => 0,
            };
            let y = match enter_edge.direction {
                Direction::Up => 0,
                Direction::Right => max_dimension - pos,
                Direction::Down => max_dimension,
                Direction::Left => pos,
            };
            Self {
                square: enter_edge.square,
                facing: enter_edge.direction.reverse(),
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct RotatedSquare {
    square: usize,
    rotation: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CubeFace {
    Top,
    Left,
    Front,
    Right,
    Back,
    Bottom,
}

#[derive(Debug, PartialEq)]
struct Cube {
    top: Option<RotatedSquare>,
    left: Option<RotatedSquare>,
    front: Option<RotatedSquare>,
    right: Option<RotatedSquare>,
    back: Option<RotatedSquare>,
    bottom: Option<RotatedSquare>,
}

impl Cube {
    fn new() -> Self {
        Self {
            top: None,
            left: None,
            front: None,
            right: None,
            back: None,
            bottom: None,
        }
    }

    fn get_edge(&self, face: CubeFace, direction: Direction) -> Edge {
        let rs = self.get_face(face).unwrap_or(RotatedSquare {
            square: 0,
            rotation: 0,
        });
        let direction = {
            let mut dir = direction;
            for _ in 0..rs.rotation {
                dir = dir.turn_left();
            }
            dir
        };

        Edge {
            direction,
            square: rs.square,
        }
    }

    fn get_face(&self, face: CubeFace) -> Option<RotatedSquare> {
        match face {
            CubeFace::Top => self.top,
            CubeFace::Left => self.left,
            CubeFace::Front => self.front,
            CubeFace::Right => self.right,
            CubeFace::Back => self.back,
            CubeFace::Bottom => self.bottom,
        }
    }

    fn set_face(&mut self, state: CubeFillState) {
        let rs = Some(RotatedSquare {
            rotation: state.rotation,
            square: state.square,
        });
        match state.face {
            CubeFace::Top => self.top = rs,
            CubeFace::Left => self.left = rs,
            CubeFace::Front => self.front = rs,
            CubeFace::Right => self.right = rs,
            CubeFace::Back => self.back = rs,
            CubeFace::Bottom => self.bottom = rs,
        }
    }
}

const STANDARD_CUBE_CONNECTIONS: [((CubeFace, Direction), (CubeFace, Direction)); 12] = [
    (
        (CubeFace::Top, Direction::Up),
        (CubeFace::Back, Direction::Up),
    ),
    (
        (CubeFace::Top, Direction::Right),
        (CubeFace::Right, Direction::Up),
    ),
    (
        (CubeFace::Top, Direction::Down),
        (CubeFace::Front, Direction::Up),
    ),
    (
        (CubeFace::Top, Direction::Left),
        (CubeFace::Left, Direction::Up),
    ),
    (
        (CubeFace::Left, Direction::Right),
        (CubeFace::Front, Direction::Left),
    ),
    (
        (CubeFace::Left, Direction::Down),
        (CubeFace::Bottom, Direction::Left),
    ),
    (
        (CubeFace::Left, Direction::Left),
        (CubeFace::Back, Direction::Right),
    ),
    (
        (CubeFace::Front, Direction::Right),
        (CubeFace::Right, Direction::Left),
    ),
    (
        (CubeFace::Front, Direction::Down),
        (CubeFace::Bottom, Direction::Up),
    ),
    (
        (CubeFace::Right, Direction::Right),
        (CubeFace::Back, Direction::Left),
    ),
    (
        (CubeFace::Right, Direction::Down),
        (CubeFace::Bottom, Direction::Right),
    ),
    (
        (CubeFace::Back, Direction::Down),
        (CubeFace::Bottom, Direction::Down),
    ),
];

#[derive(Clone, Copy, Debug, PartialEq)]
struct CubeFillState {
    square: usize,
    face: CubeFace,
    rotation: usize,
}

impl CubeFillState {
    fn neighbour_in_direction(&self, mut direction: Direction) -> (CubeFace, usize) {
        for _ in 0..self.rotation {
            direction = direction.turn_right();
        }
        match (self.face, direction) {
            (CubeFace::Top, Direction::Up) | (CubeFace::Bottom, Direction::Down) => {
                (CubeFace::Back, 2)
            }
            (CubeFace::Top, Direction::Right) => (CubeFace::Right, 1),
            (CubeFace::Top, Direction::Down) | (CubeFace::Left, Direction::Right) => {
                (CubeFace::Front, 0)
            }
            (CubeFace::Top, Direction::Left) => (CubeFace::Left, 3),
            (CubeFace::Left | CubeFace::Bottom, Direction::Up)
            | (CubeFace::Right, Direction::Left) => (CubeFace::Top, 1),
            (CubeFace::Left, Direction::Down) => (CubeFace::Bottom, 3),
            (CubeFace::Left, Direction::Left) | (CubeFace::Right, Direction::Right) => {
                (CubeFace::Back, 0)
            }
            (CubeFace::Front, Direction::Right) | (CubeFace::Back, Direction::Left) => {
                (CubeFace::Right, 0)
            }
            (CubeFace::Front, Direction::Down) => (CubeFace::Bottom, 0),
            (CubeFace::Front, Direction::Left) | (CubeFace::Back, Direction::Right) => {
                (CubeFace::Left, 0)
            }
            (CubeFace::Right, Direction::Up) => (CubeFace::Top, 3),
            (CubeFace::Right, Direction::Down) => (CubeFace::Bottom, 1),
            (_, Direction::Up) => (CubeFace::Top, 0),
            (CubeFace::Back, Direction::Down) => (CubeFace::Bottom, 2),
            (CubeFace::Bottom, Direction::Right) => (CubeFace::Right, 3),
            (CubeFace::Bottom, Direction::Left) => (CubeFace::Left, 1),
        }
    }

    fn move_to_neighbour(&self, direction: Direction, flat_neighbour: usize) -> Self {
        let (face, extra_rotation) = self.neighbour_in_direction(direction);
        Self {
            square: flat_neighbour,
            face,
            rotation: (self.rotation + extra_rotation) % 4,
        }
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

    fn square_in_direction(&self, position: Position, direction: Direction) -> Option<usize> {
        if (direction == Direction::Up && position.1 == 0)
            || (direction == Direction::Left && position.0 == 0)
        {
            None
        } else {
            self.layout
                .iter()
                .position(|pos| pos == &(position + direction))
        }
    }

    fn square_in_direction_or_wrap_around(
        &self,
        position: Position,
        direction: Direction,
    ) -> usize {
        if let Some(square) = self.square_in_direction(position, direction) {
            square
        } else {
            match direction {
                Direction::Up => self.last_square_in_column(position.0),
                Direction::Right => self.first_square_in_row(position.1),
                Direction::Down => self.first_square_in_column(position.0),
                Direction::Left => self.last_square_in_row(position.1),
            }
        }
    }

    fn get_flat_connections(&self) -> HashMap<Edge, Edge> {
        let mut connections = HashMap::new();

        for (number, position) in self.layout.iter().enumerate() {
            for direction in COMPASS {
                let edge = Edge {
                    square: number,
                    direction,
                };
                let other = self.square_in_direction_or_wrap_around(*position, direction);
                let other_edge = Edge {
                    square: other,
                    direction: direction.reverse(),
                };
                connections.insert(edge, other_edge);
            }
        }

        connections
    }

    fn assemble_cube(&self) -> Cube {
        let mut cube = Cube::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut consider = VecDeque::new();

        consider.push_back(CubeFillState {
            square: 0,
            rotation: 0,
            face: CubeFace::Front,
        });

        while let Some(state) = consider.pop_front() {
            if visited.contains(&state.square) {
                continue;
            }
            visited.insert(state.square);

            cube.set_face(state);
            if let Some(position) = self.layout.get(state.square) {
                for direction in COMPASS {
                    if let Some(flat_neighbour) = self.square_in_direction(*position, direction) {
                        consider.push_back(state.move_to_neighbour(direction, flat_neighbour));
                    }
                }
            }
        }

        cube
    }

    fn get_cube_connections(&self) -> HashMap<Edge, Edge> {
        let cube = self.assemble_cube();
        let mut connections = HashMap::new();

        for ((face_a, dir_a), (face_b, dir_b)) in STANDARD_CUBE_CONNECTIONS {
            let edge_a = cube.get_edge(face_a, dir_a);
            let edge_b = cube.get_edge(face_b, dir_b);
            connections.insert(edge_a, edge_b);
            connections.insert(edge_b, edge_a);
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

impl GroveMap {
    fn from_input(input: &str, assemble_cube: bool) -> Self {
        let (square_size, longest_line, tiles): (usize, usize, Vec<Vec<Tile>>) =
            input.lines().fold(
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

        let connections = if assemble_cube {
            layout.get_cube_connections()
        } else {
            layout.get_flat_connections()
        };

        GroveMap {
            square_size,
            squares,
            connections,
        }
    }

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

fn parse_input(
    input: &str,
    assemble_cube: bool,
) -> Result<(GroveMap, Vec<Instruction>), ParseInputError> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() == 2 {
        let map = GroveMap::from_input(parts[0], assemble_cube);

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
    if let Ok((map, instructions)) = parse_input(input, false) {
        Some(map.follow_instructions(&instructions).password(&map))
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    if let Ok((map, instructions)) = parse_input(input, true) {
        Some(map.follow_instructions(&instructions).password(&map))
    } else {
        None
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grove_map(part_two: bool) -> GroveMap {
        let connections = {
            let zero_up = Edge {
                square: 0,
                direction: Direction::Up,
            };
            let zero_right = Edge {
                square: 0,
                direction: Direction::Right,
            };
            let zero_down = Edge {
                square: 0,
                direction: Direction::Down,
            };
            let zero_left = Edge {
                square: 0,
                direction: Direction::Left,
            };

            let one_up = Edge {
                square: 1,
                direction: Direction::Up,
            };
            let one_right = Edge {
                square: 1,
                direction: Direction::Right,
            };
            let one_down = Edge {
                square: 1,
                direction: Direction::Down,
            };
            let one_left = Edge {
                square: 1,
                direction: Direction::Left,
            };

            let two_up = Edge {
                square: 2,
                direction: Direction::Up,
            };
            let two_right = Edge {
                square: 2,
                direction: Direction::Right,
            };
            let two_down = Edge {
                square: 2,
                direction: Direction::Down,
            };
            let two_left = Edge {
                square: 2,
                direction: Direction::Left,
            };

            let three_up = Edge {
                square: 3,
                direction: Direction::Up,
            };
            let three_right = Edge {
                square: 3,
                direction: Direction::Right,
            };
            let three_down = Edge {
                square: 3,
                direction: Direction::Down,
            };
            let three_left = Edge {
                square: 3,
                direction: Direction::Left,
            };

            let four_up = Edge {
                square: 4,
                direction: Direction::Up,
            };
            let four_right = Edge {
                square: 4,
                direction: Direction::Right,
            };
            let four_down = Edge {
                square: 4,
                direction: Direction::Down,
            };
            let four_left = Edge {
                square: 4,
                direction: Direction::Left,
            };

            let five_up = Edge {
                square: 5,
                direction: Direction::Up,
            };
            let five_right = Edge {
                square: 5,
                direction: Direction::Right,
            };
            let five_down = Edge {
                square: 5,
                direction: Direction::Down,
            };
            let five_left = Edge {
                square: 5,
                direction: Direction::Left,
            };

            let mut connections = HashMap::new();

            if part_two {
                connections.insert(zero_up, one_up);
                connections.insert(zero_right, five_right);
                connections.insert(zero_down, three_up);
                connections.insert(zero_left, two_up);

                connections.insert(one_up, zero_up);
                connections.insert(one_right, two_left);
                connections.insert(one_down, four_down);
                connections.insert(one_left, five_down);

                connections.insert(two_up, zero_left);
                connections.insert(two_right, three_left);
                connections.insert(two_down, four_left);
                connections.insert(two_left, one_right);

                connections.insert(three_up, zero_down);
                connections.insert(three_right, five_up);
                connections.insert(three_down, four_up);
                connections.insert(three_left, two_right);

                connections.insert(four_up, three_down);
                connections.insert(four_right, five_left);
                connections.insert(four_down, one_down);
                connections.insert(four_left, two_down);

                connections.insert(five_up, three_right);
                connections.insert(five_right, zero_right);
                connections.insert(five_down, one_left);
                connections.insert(five_left, four_right);
            } else {
                connections.insert(zero_up, four_down);
                connections.insert(zero_right, zero_left);
                connections.insert(zero_down, three_up);
                connections.insert(zero_left, zero_right);

                connections.insert(one_up, one_down);
                connections.insert(one_right, two_left);
                connections.insert(one_down, one_up);
                connections.insert(one_left, three_right);

                connections.insert(two_up, two_down);
                connections.insert(two_right, three_left);
                connections.insert(two_down, two_up);
                connections.insert(two_left, one_right);

                connections.insert(three_up, zero_down);
                connections.insert(three_right, one_left);
                connections.insert(three_down, four_up);
                connections.insert(three_left, two_right);

                connections.insert(four_up, three_down);
                connections.insert(four_right, five_left);
                connections.insert(four_down, zero_up);
                connections.insert(four_left, five_right);

                connections.insert(five_up, five_down);
                connections.insert(five_right, four_left);
                connections.insert(five_down, five_up);
                connections.insert(five_left, four_right);
            }

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
    fn test_parse_input_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        let expected = Ok((
            example_grove_map(false),
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
        assert_eq!(parse_input(&input, false), expected);
    }

    #[test]
    fn test_wrap_around() {
        let map = example_grove_map(false);
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
    fn test_state_directions() {
        let front = CubeFillState {
            face: CubeFace::Front,
            rotation: 0,
            square: 0,
        };
        let down = CubeFillState {
            face: CubeFace::Bottom,
            rotation: 0,
            square: 3,
        };
        let down_left = CubeFillState {
            face: CubeFace::Left,
            rotation: 1,
            square: 2,
        };
        let down_left_left = CubeFillState {
            face: CubeFace::Top,
            rotation: 2,
            square: 1,
        };
        let down_down = CubeFillState {
            face: CubeFace::Back,
            rotation: 2,
            square: 4,
        };
        let down_down_right = CubeFillState {
            face: CubeFace::Right,
            rotation: 2,
            square: 5,
        };
        assert_eq!(front.move_to_neighbour(Direction::Down, 3), down);
        assert_eq!(down.move_to_neighbour(Direction::Left, 2), down_left);
        assert_eq!(
            down_left.move_to_neighbour(Direction::Left, 1),
            down_left_left
        );
        assert_eq!(down.move_to_neighbour(Direction::Down, 4), down_down);
        assert_eq!(
            down_down.move_to_neighbour(Direction::Right, 5),
            down_down_right
        );
    }

    #[test]
    fn test_assemble_cube() {
        let layout = GroveLayout {
            layout: vec![
                Position(2, 0),
                Position(0, 1),
                Position(1, 1),
                Position(2, 1),
                Position(2, 2),
                Position(3, 2),
            ],
        };
        assert_eq!(
            layout.assemble_cube(),
            Cube {
                top: Some(RotatedSquare {
                    square: 1,
                    rotation: 2
                }),
                left: Some(RotatedSquare {
                    square: 2,
                    rotation: 1
                }),
                front: Some(RotatedSquare {
                    square: 0,
                    rotation: 0
                }),
                right: Some(RotatedSquare {
                    square: 5,
                    rotation: 2
                }),
                back: Some(RotatedSquare {
                    square: 4,
                    rotation: 2
                }),
                bottom: Some(RotatedSquare {
                    square: 3,
                    rotation: 0
                }),
            }
        );
    }

    #[test]
    fn test_parse_input_part_two() {
        let input = advent_of_code::read_file("examples", 22);

        let expected = Ok((
            example_grove_map(true),
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
        assert_eq!(parse_input(&input, true), expected);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(5031));
    }
}
