use std::collections::{HashMap, HashSet};
use std::iter::repeat;

const MAX_X: u64 = 6;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(u64, u64);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Minus,
    Plus,
    Angle,
    Pole,
    Square,
}

fn shape_points(shape_type: Shape, bottom_left: Point) -> Vec<Point> {
    let Point(x, y) = bottom_left;
    match shape_type {
        Shape::Minus => vec![
            Point(x, y),
            Point(x + 1, y),
            Point(x + 2, y),
            Point(x + 3, y),
        ],
        Shape::Plus => vec![
            Point(x + 1, y),
            Point(x, y + 1),
            Point(x + 1, y + 1),
            Point(x + 2, y + 1),
            Point(x + 1, y + 2),
        ],
        Shape::Angle => vec![
            Point(x, y),
            Point(x + 1, y),
            Point(x + 2, y),
            Point(x + 2, y + 1),
            Point(x + 2, y + 2),
        ],
        Shape::Pole => vec![
            Point(x, y),
            Point(x, y + 1),
            Point(x, y + 2),
            Point(x, y + 3),
        ],
        Shape::Square => vec![
            Point(x, y),
            Point(x + 1, y),
            Point(x, y + 1),
            Point(x + 1, y + 1),
        ],
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            _ => Direction::Right,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct TetrisCycleState {
    jet_ix: usize,
    shape_ix: usize,
    y_histogram: Vec<u64>,
}

impl TetrisCycleState {
    fn from_game(game: &TetrisGame) -> Self {
        let max_y = game.max_y();
        let histogram = game.max_y_values.iter().map(|val| max_y - val).collect();
        TetrisCycleState {
            jet_ix: game.jet_ix,
            shape_ix: game.shape_ix % game.shapes.len(),
            y_histogram: histogram,
        }
    }
}

#[derive(Debug, PartialEq)]
struct TetrisCycleData {
    first_seen: usize,
    second_seen: usize,
    height_change: u64,
}

#[derive(Debug, PartialEq)]
enum TetrisCycle {
    None,
    Detected(TetrisCycleData),
    Removed,
}

#[derive(Debug)]
struct TetrisGame {
    jets: Vec<Direction>,
    jet_ix: usize,
    current_shape: Vec<Point>,
    shapes: Vec<Shape>,
    shape_ix: usize,
    occupied: HashSet<Point>,
    max_y_values: Vec<u64>,
    visited: HashMap<TetrisCycleState, (usize, u64)>,
    cycle: TetrisCycle,
}

impl TetrisGame {
    fn new(input: &str) -> Self {
        let jets = input.trim().chars().map(Direction::from_char).collect();
        let shapes = vec![
            Shape::Minus,
            Shape::Plus,
            Shape::Angle,
            Shape::Pole,
            Shape::Square,
        ];
        Self {
            jets,
            jet_ix: 0,
            current_shape: shape_points(Shape::Minus, Point(2, 4)),
            shapes,
            shape_ix: 0,
            occupied: HashSet::new(),
            max_y_values: repeat(0).take((MAX_X + 1) as usize).collect(),
            visited: HashMap::new(),
            cycle: TetrisCycle::None,
        }
    }

    fn max_y(&self) -> u64 {
        self.max_y_values.iter().fold(0, |max, y| max.max(*y))
    }

    fn next_jet(&mut self) -> Direction {
        let jet = self.jets[self.jet_ix];
        self.jet_ix = (self.jet_ix + 1) % self.jets.len();
        jet
    }

    fn next_shape(&mut self) {
        for pt in &self.current_shape {
            let x = pt.0 as usize;
            self.max_y_values[x] = self.max_y_values[x].max(pt.1);
            self.occupied.insert(*pt);
        }
        self.shape_ix += 1;
        self.current_shape = shape_points(
            self.shapes[self.shape_ix % self.shapes.len()],
            Point(2, self.max_y() + 4),
        );

        if self.cycle == TetrisCycle::None {
            let state = TetrisCycleState::from_game(self);
            let height = self.max_y();
            if let Some((first_seen, first_height)) = self.visited.get(&state) {
                self.cycle = TetrisCycle::Detected(TetrisCycleData {
                    first_seen: *first_seen,
                    second_seen: self.shape_ix,
                    height_change: height - first_height,
                });
            } else {
                self.visited.insert(state, (self.shape_ix, height));
            }
        }
    }

    fn try_move(&mut self, direction: Direction) {
        let moved: Vec<Point> = self
            .current_shape
            .iter()
            .filter_map(|pt| {
                if (pt.0 == 0 && direction == Direction::Left)
                    || (pt.0 == MAX_X && direction == Direction::Right)
                    || (pt.1 == 1 && direction == Direction::Down)
                {
                    None
                } else {
                    let moved_pt = match direction {
                        Direction::Left => Point(pt.0 - 1, pt.1),
                        Direction::Right => Point(pt.0 + 1, pt.1),
                        Direction::Down => Point(pt.0, pt.1 - 1),
                    };
                    if self.occupied.contains(&moved_pt) {
                        None
                    } else {
                        Some(moved_pt)
                    }
                }
            })
            .collect();
        if moved.len() == self.current_shape.len() {
            self.current_shape = moved;
        } else if direction == Direction::Down {
            self.next_shape();
        }
    }

    fn tick(&mut self) {
        let direction = self.next_jet();
        self.try_move(direction);
        self.try_move(Direction::Down);
    }

    fn height_after_rocks(&mut self, shapes: usize) -> u64 {
        let mut extra_height = 0;

        while self.shape_ix < shapes {
            self.tick();

            if let TetrisCycle::Detected(cycle) = &self.cycle {
                let cycle_length = cycle.second_seen - cycle.first_seen;
                let add_cycles = (shapes - cycle.second_seen) / cycle_length;
                extra_height += add_cycles as u64 * cycle.height_change;
                self.shape_ix += add_cycles * cycle_length;
                self.cycle = TetrisCycle::Removed
            }
        }

        self.max_y() + extra_height
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut game = TetrisGame::new(input);
    Some(game.height_after_rocks(2022))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut game = TetrisGame::new(input);
    Some(game.height_after_rocks(1_000_000_000_000))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_minus() {
        assert_eq!(
            shape_points(Shape::Minus, Point(2, 0)),
            vec![Point(2, 0), Point(3, 0), Point(4, 0), Point(5, 0),]
        );
    }

    #[test]
    fn test_shape_plus() {
        assert_eq!(
            shape_points(Shape::Plus, Point(1, 2)),
            vec![
                Point(2, 2),
                Point(1, 3),
                Point(2, 3),
                Point(3, 3),
                Point(2, 4),
            ]
        );
    }

    #[test]
    fn test_shape_angle() {
        assert_eq!(
            shape_points(Shape::Angle, Point(3, 4)),
            vec![
                Point(3, 4),
                Point(4, 4),
                Point(5, 4),
                Point(5, 5),
                Point(5, 6),
            ]
        );
    }

    #[test]
    fn test_shape_pole() {
        assert_eq!(
            shape_points(Shape::Pole, Point(5, 4)),
            vec![Point(5, 4), Point(5, 5), Point(5, 6), Point(5, 7),]
        );
    }

    #[test]
    fn test_shape_square() {
        assert_eq!(
            shape_points(Shape::Square, Point(3, 4)),
            vec![Point(3, 4), Point(4, 4), Point(3, 5), Point(4, 5),]
        );
    }

    #[test]
    fn test_cycle_directions() {
        let mut game = TetrisGame::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(game.next_jet(), Direction::Right);
        assert_eq!(game.next_jet(), Direction::Right);
        assert_eq!(game.next_jet(), Direction::Right);
        assert_eq!(game.next_jet(), Direction::Left);
        assert_eq!(game.next_jet(), Direction::Left);
        assert_eq!(game.next_jet(), Direction::Right);
        assert_eq!(game.next_jet(), Direction::Left);
        assert_eq!(game.next_jet(), Direction::Right);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1_514_285_714_288));
    }
}
