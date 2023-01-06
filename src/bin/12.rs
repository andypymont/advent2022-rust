use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum ShortestPathType {
    EndToEnd,
    Hiking,
}

#[derive(Debug, PartialEq)]
struct Grid {
    width: usize,
    heights: Vec<u32>,
    start: usize,
    goal: usize,
}

impl Grid {
    fn shortest_path(&self, path_type: &ShortestPathType) -> Option<u32> {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut consider: VecDeque<(usize, u32)> = VecDeque::new();
        consider.push_back((self.goal, 0));

        while let Some((pos, steps)) = consider.pop_front() {
            let height = self.heights[pos];

            match path_type {
                ShortestPathType::EndToEnd => {
                    if pos == self.start {
                        return Some(steps);
                    }
                }
                ShortestPathType::Hiking => {
                    if height == 0 {
                        return Some(steps);
                    }
                }
            }

            if visited.contains(&pos) {
                continue;
            };

            visited.insert(pos);

            let min_height = if height == 0 { 0 } else { height - 1 };
            let x = pos % self.width;

            if x != 0 {
                let left = pos - 1;
                if self.heights[left] >= min_height {
                    consider.push_back((left, steps + 1));
                }
            }
            if x + 1 != self.width {
                let right = pos + 1;
                if self.heights[right] >= min_height {
                    consider.push_back((right, steps + 1));
                }
            }
            if pos >= self.width {
                let up = pos - self.width;
                if self.heights[up] >= min_height {
                    consider.push_back((up, steps + 1));
                }
            }
            let down = pos + self.width;
            if down < self.heights.len() && self.heights[down] >= min_height {
                consider.push_back((down, steps + 1));
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width: Result<usize, ParseGridError> = Err(ParseGridError);
        let mut heights = Vec::new();
        let mut start: Result<usize, ParseGridError> = Err(ParseGridError);
        let mut goal: Result<usize, ParseGridError> = Err(ParseGridError);

        for line in s.lines() {
            width = match width {
                Ok(current) => {
                    if line.len() != current {
                        Err(ParseGridError)
                    } else {
                        Ok(current)
                    }
                }
                Err(_) => Ok(line.len()),
            };
            for ch in line.chars() {
                let pos = heights.len();
                match ch {
                    'S' => {
                        heights.push(0);
                        start = Ok(pos);
                    }
                    'E' => {
                        heights.push(25);
                        goal = Ok(pos);
                    }
                    _ => {
                        heights.push(ch.to_digit(36).unwrap_or(10) - 10);
                    }
                };
            }
        }

        Ok(Grid {
            width: width?,
            heights,
            start: start?,
            goal: goal?,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Ok(grid) = input.parse::<Grid>() {
        grid.shortest_path(&ShortestPathType::EndToEnd)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    if let Ok(grid) = input.parse::<Grid>() {
        grid.shortest_path(&ShortestPathType::Hiking)
    } else {
        None
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_grid() {
        let input = concat![
            "Sabqponm\n",
            "abcryxxl\n",
            "accszExk\n",
            "acctuvwj\n",
            "abdefghi\n",
        ];
        let grid = Grid {
            width: 8,
            heights: vec![
                0, 0, 1, 16, 15, 14, 13, 12, 0, 1, 2, 17, 24, 23, 23, 11, 0, 2, 2, 18, 25, 25, 23,
                10, 0, 2, 2, 19, 20, 21, 22, 9, 0, 1, 3, 4, 5, 6, 7, 8,
            ],
            start: 0,
            goal: 21,
        };
        assert_eq!(input.parse(), Ok(grid));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
