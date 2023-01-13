use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Range {
    start: u32,
    finish: u32,
}

#[derive(Debug, PartialEq)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() == 2 {
            let start: u32 = parts[0].parse().map_err(|_| ParseRangeError)?;
            let finish: u32 = parts[1].parse().map_err(|_| ParseRangeError)?;
            Ok(Range { start, finish })
        } else {
            Err(ParseRangeError)            
        }
    }
}

impl Range {
    fn is_fully_contained_by_other(&self, other: &Range) -> bool {
        self.start >= other.start && self.finish <= other.finish
    }

    fn has_overlap_with_other(&self, other: &Range) -> bool {
        (self.start >= other.start && self.start <= other.finish)
            || (self.finish >= other.start && self.finish <= other.finish)
    }
}

#[derive(Debug, PartialEq)]
struct Pair {
    first: Range,
    second: Range,
}

impl FromStr for Pair {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<&str> = s.split(',').collect();
        if ranges.len() == 2 {
            let first: Range = ranges[0].parse()?;
            let second: Range = ranges[1].parse()?;
            Ok(Pair { first, second })            
        } else {
            Err(ParseRangeError)
        }
    }
}

impl Pair {
    fn is_overlapping(&self) -> bool {
        self.first.has_overlap_with_other(&self.second)
            || self.second.has_overlap_with_other(&self.first)
    }

    fn is_fully_overlapping(&self) -> bool {
        self.first.is_fully_contained_by_other(&self.second)
            || self.second.is_fully_contained_by_other(&self.first)
    }
}

fn read_pairs(input: &str) -> Result<Vec<Pair>, ParseRangeError> {
    let mut pairs: Vec<Pair> = Vec::new();
    for line in input.lines() {
        let pair: Result<Pair, ParseRangeError> = line.parse();
        match pair {
            Err(e) => return Err(e),
            Ok(p) => pairs.push(p),
        };
    }
    Ok(pairs)
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    match read_pairs(input) {
        Err(_) => None,
        Ok(pairs) => Some(
            pairs
                .iter()
                .map(|pair| u32::from(pair.is_fully_overlapping()))
                .sum(),
        ),
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    match read_pairs(input) {
        Err(_) => None,
        Ok(pairs) => Some(
            pairs
                .iter()
                .map(|pair| u32::from(pair.is_overlapping()))
                .sum(),
        ),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_pairs() {
        let input = advent_of_code::read_file("examples", 4);

        assert_eq!(
            read_pairs(&input),
            Ok(vec![
                Pair {
                    first: Range {
                        start: 2,
                        finish: 4
                    },
                    second: Range {
                        start: 6,
                        finish: 8
                    },
                },
                Pair {
                    first: Range {
                        start: 2,
                        finish: 3
                    },
                    second: Range {
                        start: 4,
                        finish: 5
                    },
                },
                Pair {
                    first: Range {
                        start: 5,
                        finish: 7
                    },
                    second: Range {
                        start: 7,
                        finish: 9
                    },
                },
                Pair {
                    first: Range {
                        start: 2,
                        finish: 8
                    },
                    second: Range {
                        start: 3,
                        finish: 7
                    },
                },
                Pair {
                    first: Range {
                        start: 6,
                        finish: 6
                    },
                    second: Range {
                        start: 4,
                        finish: 6
                    },
                },
                Pair {
                    first: Range {
                        start: 2,
                        finish: 6
                    },
                    second: Range {
                        start: 4,
                        finish: 8
                    },
                },
            ])
        );
    }

    #[test]
    fn test_pair_four_fully_overlaps() {
        let pair = Pair {
            first: Range {
                start: 2,
                finish: 8,
            },
            second: Range {
                start: 3,
                finish: 7,
            },
        };
        assert_eq!(pair.is_fully_overlapping(), true);
    }

    #[test]
    fn test_pair_one_doesnt_fully_overlap() {
        let pair = Pair {
            first: Range {
                start: 2,
                finish: 4,
            },
            second: Range {
                start: 6,
                finish: 8,
            },
        };
        assert_eq!(pair.is_fully_overlapping(), false);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_pair_one_doesnt_overlap() {
        let pair = Pair {
            first: Range {
                start: 2,
                finish: 4,
            },
            second: Range {
                start: 6,
                finish: 8,
            },
        };
        assert_eq!(pair.is_overlapping(), false);
    }

    #[test]
    fn test_pair_three_overlaps() {
        let pair = Pair {
            first: Range {
                start: 5,
                finish: 7,
            },
            second: Range {
                start: 7,
                finish: 9,
            },
        };
        assert_eq!(pair.is_overlapping(), true);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
