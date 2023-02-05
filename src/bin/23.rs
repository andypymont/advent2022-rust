use std::collections::BTreeMap;
use std::str::FromStr;

const GRID_SIZE: usize = 400;

fn neighbours(pos: usize) -> [usize; 8] {
    [
        pos - GRID_SIZE - 1, // NW
        pos - GRID_SIZE,     // N
        pos - GRID_SIZE + 1, // NE
        pos + 1,             // E
        pos + GRID_SIZE + 1, // SE
        pos + GRID_SIZE,     // S
        pos + GRID_SIZE - 1, // SW
        pos - 1,             // W
    ]
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
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
}

const DIRECTION_CYCLE: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

fn moved_pos(pos: usize, dir: &Direction, occupied: u8) -> Option<usize> {
    if dir.check_neighbours() & occupied == 0 {
        Some(match dir {
            Direction::North => pos - GRID_SIZE,
            Direction::South => pos + GRID_SIZE,
            Direction::West => pos - 1,
            Direction::East => pos + 1,
        })
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
struct State {
    grid: Vec<bool>,
    rounds: usize,
}

struct ParseStateError;

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![false; GRID_SIZE * GRID_SIZE];

        let base = GRID_SIZE / 2;
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let pos = {
                        let y = y + base;
                        let x = x + base;
                        (y * GRID_SIZE) + x
                    };
                    grid[pos] = true;
                }
            }
        }

        Ok(Self { grid, rounds: 0 })
    }
}

impl State {
    fn direction_checks(&self) -> [Direction; 4] {
        let mut cycle = DIRECTION_CYCLE;
        cycle.rotate_left(self.rounds % 4);
        cycle
    }

    fn enclosed_empty_spaces(&self) -> usize {
        let mut elves = 0;
        let (mut left, mut right, mut top, mut bottom) =
            (usize::MAX, usize::MIN, usize::MAX, usize::MIN);

        for (pos, is_elf) in self.grid.iter().enumerate() {
            if *is_elf {
                let (x, y) = (pos % GRID_SIZE, pos / GRID_SIZE);
                left = left.min(x);
                right = right.max(x);
                top = top.min(y);
                bottom = bottom.max(y);

                elves += 1;
            }
        }

        if (left > right) || (top > bottom) {
            0
        } else {
            ((bottom - top + 1) * (right - left + 1)) - elves
        }
    }

    fn next_round(&mut self) -> usize {
        let checks = self.direction_checks();
        let mut proposed: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

        self.grid.iter().enumerate().for_each(|(pos, is_elf)| {
            if *is_elf {
                let occupied = self.occupied_neighbours(pos);
                if occupied != 0 {
                    if let Some(dest) = checks.iter().find_map(|dir| moved_pos(pos, dir, occupied))
                    {
                        proposed.entry(dest).or_insert_with(Vec::new).push(pos);
                    }
                }
            }
        });

        let mut latest_moves = 0;
        for (dest, elves) in proposed {
            if elves.len() == 1 {
                let from = elves[0];
                self.grid[from] = false;
                self.grid[dest] = true;
                latest_moves += 1;
            }
        }

        self.rounds += 1;
        latest_moves
    }

    fn occupied_neighbours(&self, pos: usize) -> u8 {
        neighbours(pos)
            .iter()
            .enumerate()
            .filter_map(|(ix, pos)| {
                if self.grid[*pos] {
                    if let Ok(ix) = u8::try_from(ix) {
                        Some(1 << ix)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .fold(0, |a, b| a | b)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    if let Ok(mut state) = input.parse::<State>() {
        while state.rounds < 10 {
            state.next_round();
        }
        Some(state.enclosed_empty_spaces())
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    if let Ok(mut state) = input.parse::<State>() {
        while state.next_round() != 0 {
            continue;
        }
        Some(state.rounds)
    } else {
        None
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
