use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Elf(i32, i32);

impl Elf {
    fn max(self, other: &Elf) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }

    fn min(self, other: &Elf) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }

    fn neighbours(&self) -> [Self; 8] {
        [
            Self(self.0 - 1, self.1 - 1),
            Self(self.0, self.1 - 1),
            Self(self.0 + 1, self.1 - 1),
            Self(self.0 + 1, self.1),
            Self(self.0 + 1, self.1 + 1),
            Self(self.0, self.1 + 1),
            Self(self.0 - 1, self.1 + 1),
            Self(self.0 - 1, self.1),
        ]
    }

    fn occupied_neighbours(&self, elves: &HashSet<Elf>) -> u8 {
        self.neighbours()
            .iter()
            .enumerate()
            .filter_map(|(ix, elf)| {
                if elves.contains(elf) {
                    if let Ok(ix) = u8::try_from(ix) {
                        Some(1 << ix)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn check_neighbours(&self) -> u8 {
        match self {
            Direction::North => 7,
            Direction::South => 112,
            Direction::West => 193,
            Direction::East => 28,
        }
    }

    fn from_cycle_position(cycle_pos: usize) -> Direction {
        match cycle_pos % 4 {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::East,
        }
    }

    fn move_elf(&self, elf: &Elf, occupied_neighbours: u8) -> Option<Elf> {
        if self.check_neighbours() & occupied_neighbours == 0 {
            Some(match self {
                Direction::North => Elf(elf.0, elf.1 - 1),
                Direction::South => Elf(elf.0, elf.1 + 1),
                Direction::West => Elf(elf.0 - 1, elf.1),
                Direction::East => Elf(elf.0 + 1, elf.1),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseStateError;

#[derive(Debug, PartialEq)]
struct State {
    direction_cycle: usize,
    elves: HashSet<Elf>,
}

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    elves.insert(Elf(
                        i32::try_from(x).unwrap_or(0),
                        i32::try_from(y).unwrap_or(0),
                    ));
                }
            }
        }

        Ok(Self {
            direction_cycle: 0,
            elves,
        })
    }
}

impl State {
    fn direction_checks(&self) -> [Direction; 4] {
        [
            Direction::from_cycle_position(self.direction_cycle),
            Direction::from_cycle_position(self.direction_cycle + 1),
            Direction::from_cycle_position(self.direction_cycle + 2),
            Direction::from_cycle_position(self.direction_cycle + 3),
        ]
    }

    fn enclosed_empty_spaces(self) -> u32 {
        let (top_left, bottom_right) = self
            .elves
            .iter()
            .fold((Elf(0, 0), Elf(0, 0)), |(top_left, bottom_right), elf| {
                (top_left.min(elf), bottom_right.max(elf))
            });

        let height = u32::try_from(bottom_right.1 - top_left.1 + 1).unwrap_or(0);
        let width = u32::try_from(bottom_right.0 - top_left.0 + 1).unwrap_or(0);
        let elf_count = u32::try_from(self.elves.len()).unwrap_or(0);
        (width * height) - elf_count
    }

    fn next(&self) -> Self {
        let checks = self.direction_checks();
        let mut counter = HashMap::new();
        let elf_proposals: Vec<(Elf, Elf)> = self
            .elves
            .iter()
            .map(|elf| {
                let occupied = elf.occupied_neighbours(&self.elves);
                if occupied == 0 {
                    (*elf, *elf)
                } else {
                    if let Some(dest) = checks
                        .iter()
                        .filter_map(|dir| dir.move_elf(elf, occupied))
                        .next()
                    {
                        counter.entry(dest).and_modify(|e| *e += 1).or_insert(1);
                        (*elf, dest)
                    } else {
                        (*elf, *elf)
                    }
                }
            })
            .collect();
        Self {
            direction_cycle: (self.direction_cycle + 1) % 4,
            elves: elf_proposals
                .iter()
                .map(|(elf, dest)| {
                    if counter.get(dest).unwrap_or(&1) > &1 {
                        *elf
                    } else {
                        *dest
                    }
                })
                .collect(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(mut state) = input.parse::<State>() {
        for _ in 0..10 {
            state = state.next()
        }
        Some(state.enclosed_empty_spaces())
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! set {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_set = HashSet::new();
                $(
                    temp_set.insert($x);
                )*
                temp_set
            }
        };
    }

    #[test]
    fn test_parse_initial_state() {
        let input = concat![".....\n", "..##.\n", "..#..\n", ".....\n", "..##.\n", ".....\n",];
        assert_eq!(
            input.parse(),
            Ok(State {
                direction_cycle: 0,
                elves: set![Elf(2, 1), Elf(3, 1), Elf(2, 2), Elf(2, 4), Elf(3, 4)],
            })
        );
    }

    #[test]
    fn test_occupied_neighbours() {
        let state = State {
            direction_cycle: 0,
            elves: set![Elf(2, 1), Elf(3, 1), Elf(2, 2), Elf(2, 4), Elf(3, 4)],
        };
        assert_eq!(
            Elf(2, 1).occupied_neighbours(&state.elves),
            40, // E & S
        );
        assert_eq!(
            Elf(3, 1).occupied_neighbours(&state.elves),
            192, // W & SW
        );
        assert_eq!(
            Elf(2, 4).occupied_neighbours(&state.elves),
            8, // E
        );
        assert_eq!(
            Elf(0, 0).occupied_neighbours(&state.elves),
            0, // None
        );
    }

    #[test]
    fn test_state_next() {
        let zero = State {
            direction_cycle: 0,
            elves: set![Elf(2, 1), Elf(3, 1), Elf(2, 2), Elf(2, 4), Elf(3, 4)],
        };
        let one = State {
            direction_cycle: 1,
            elves: set![Elf(2, 0), Elf(3, 0), Elf(2, 2), Elf(3, 3), Elf(2, 4)],
        };
        let two = State {
            direction_cycle: 2,
            elves: set![Elf(2, 1), Elf(3, 1), Elf(1, 2), Elf(4, 3), Elf(2, 5)],
        };
        let three = State {
            direction_cycle: 3,
            elves: set![Elf(2, 0), Elf(4, 1), Elf(0, 2), Elf(4, 3), Elf(2, 5)],
        };
        let four = State {
            direction_cycle: 0,
            elves: set![Elf(2, 0), Elf(4, 1), Elf(0, 2), Elf(4, 3), Elf(2, 5)],
        };
        assert_eq!(zero.next(), one);
        assert_eq!(one.next(), two);
        assert_eq!(two.next(), three);
        assert_eq!(three.next(), four);
    }

    #[test]
    fn test_enclosed_empty_spaces() {
        let state = State {
            direction_cycle: 3,
            elves: set![
                Elf(6, 0),
                Elf(4, 1),
                Elf(9, 1),
                Elf(1, 2),
                Elf(4, 2),
                Elf(8, 2),
                Elf(6, 3),
                Elf(10, 3),
                Elf(2, 4),
                Elf(5, 4),
                Elf(7, 4),
                Elf(0, 5),
                Elf(3, 5),
                Elf(9, 5),
                Elf(6, 6),
                Elf(7, 6),
                Elf(1, 7),
                Elf(2, 7),
                Elf(4, 7),
                Elf(9, 7),
                Elf(2, 8),
                Elf(6, 9)
            ],
        };
        assert_eq!(state.enclosed_empty_spaces(), 88);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), None);
    }
}
