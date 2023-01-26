use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct State {
    width: usize,
    height: usize,
    time: u32,
    obstacles: Vec<u32>,
    elf: Vec<bool>,
    goal: usize,
}

#[derive(Debug, PartialEq)]
struct ParseStateError;

const WALL: u32 = 1;
const BLIZZARD_U: u32 = 2;
const BLIZZARD_R: u32 = 4;
const BLIZZARD_D: u32 = 8;
const BLIZZARD_L: u32 = 16;

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len() - 2;
        let width = lines.first().unwrap_or(&"  ").len() - 2;

        let mut obstacles = Vec::new();
        let mut elf = Vec::new();

        for _ in 0..(width + 2) {
            obstacles.push(WALL);
            elf.push(false);
        }
        for line in lines {
            for ch in line.chars() {
                obstacles.push(match ch {
                    '#' => WALL,
                    '^' => BLIZZARD_U,
                    '>' => BLIZZARD_R,
                    'v' => BLIZZARD_D,
                    '<' => BLIZZARD_L,
                    _ => 0,
                });
                elf.push(false);
            }
        }
        for _ in 0..(width + 2) {
            obstacles.push(WALL);
            elf.push(false);
        }

        elf[width + 3] = true;

        Ok(Self {
            width,
            height,
            time: 0,
            obstacles,
            elf,
            goal: ((width + 2) * (height + 2)) + width,
        })
    }
}

impl State {
    fn advance(&mut self) {
        self.time += 1;
        let total_width = self.width + 2;
        let total_height = self.height + 2;

        for (pos, ob) in self.obstacles.clone().iter().enumerate() {
            let (y, x) = (pos / total_width, pos % total_width);
            if ob & BLIZZARD_U == BLIZZARD_U {
                self.obstacles[pos] -= BLIZZARD_U;
                let up = (if y == 2 { self.height + 1 } else { y - 1 } * total_width) + x;
                self.obstacles[up] += BLIZZARD_U;
            }
            if ob & BLIZZARD_R == BLIZZARD_R {
                self.obstacles[pos] -= BLIZZARD_R;
                let right = (y * total_width) + if x == self.width { 1 } else { x + 1 };
                self.obstacles[right] += BLIZZARD_R;
            }
            if ob & BLIZZARD_D == BLIZZARD_D {
                self.obstacles[pos] -= BLIZZARD_D;
                let down = (if y == self.height + 1 { 2 } else { y + 1 } * total_width) + x;
                self.obstacles[down] += BLIZZARD_D;
            }
            if ob & BLIZZARD_L == BLIZZARD_L {
                self.obstacles[pos] -= BLIZZARD_L;
                let left = (y * total_width) + if x == 1 { self.width } else { x - 1 };
                self.obstacles[left] += BLIZZARD_L;
            }
        }

        for (pos, elf) in self.elf.clone().iter().enumerate() {
            let (y, x) = (pos / total_width, pos % total_width);
            if *elf {
                self.elf[pos] = self.obstacles[pos] == 0;
                if y > 0 {
                    let up = pos - total_width;
                    self.elf[up] = self.obstacles[up] == 0;
                }
                if y < total_height {
                    let down = pos + total_width;
                    self.elf[down] = self.obstacles[down] == 0;
                }
                if x > 0 {
                    let left = pos - 1;
                    self.elf[left] = self.obstacles[left] == 0;
                }
                if x < (total_width - 1) {
                    let right = pos + 1;
                    self.elf[right] = self.obstacles[right] == 0;
                }
            }
        }
    }

    fn clear_elf_positions(&mut self) {
        self.elf = vec![false; self.obstacles.len()];
    }

    fn has_elf_reached(&self, pos: usize) -> bool {
        self.elf[pos]
    }

    fn is_solved(&self) -> bool {
        self.has_elf_reached(self.goal)
    }

    fn reset_for_trip(&mut self, trip: usize) {
        self.clear_elf_positions();

        let (start, goal) = {
            let entrance = self.width + 3;
            let other_side = ((self.width + 2) * (self.height + 2)) + self.width;

            match trip % 2 {
                1 => (other_side, entrance),
                _ => (entrance, other_side),
            }
        };

        self.elf[start] = true;
        self.goal = goal;
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(mut state) = input.parse::<State>() {
        while !state.is_solved() {
            state.advance();
        }
        Some(state.time)
    } else {
        None
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    if let Ok(mut state) = input.parse::<State>() {
        while !state.is_solved() {
            state.advance();
        }
        state.reset_for_trip(1);
        while !state.is_solved() {
            state.advance();
        }
        state.reset_for_trip(2);
        while !state.is_solved() {
            state.advance();
        }

        Some(state.time)
    } else {
        None
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(
            input.parse(),
            Ok(State {
                width: 6,
                height: 4,
                time: 0,
                obstacles: vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 4, 4, 0, 16, 2, 16, 1, 1, 0,
                    16, 0, 0, 16, 16, 1, 1, 4, 8, 0, 4, 16, 4, 1, 1, 16, 2, 8, 2, 2, 4, 1, 1, 1, 1,
                    1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
                elf: vec![
                    false, false, false, false, false, false, false, false, false, true, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false,
                ],
                goal: 54,
            })
        )
    }

    #[test]
    fn test_advance() {
        let mut initial = State {
            width: 6,
            height: 4,
            time: 0,
            obstacles: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 4, 4, 0, 16, 2, 16, 1, 1, 0, 16,
                0, 0, 16, 16, 1, 1, 4, 8, 0, 4, 16, 4, 1, 1, 16, 2, 8, 2, 2, 4, 1, 1, 1, 1, 1, 1,
                1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            elf: vec![
                false, false, false, false, false, false, false, false, false, true, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            goal: 54,
        };
        let one = State {
            width: 6,
            height: 4,
            time: 1,
            obstacles: vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 4, 28, 0, 16, 0, 1, 1, 16, 0,
                0, 16, 16, 0, 1, 1, 4, 6, 0, 18, 6, 0, 1, 1, 4, 8, 0, 0, 2, 16, 1, 1, 1, 1, 1, 1,
                1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            elf: vec![
                false, false, false, false, false, false, false, false, false, true, false, false,
                false, false, false, false, false, true, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            goal: 54,
        };
        initial.advance();
        assert_eq!(initial, one,);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
