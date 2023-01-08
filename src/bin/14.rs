use std::collections::HashSet;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32);

#[derive(Debug, PartialEq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            Err(ParsePointError)
        } else {
            Ok(Point(
                parts[0].parse().map_err(|_| ParsePointError)?,
                parts[1].parse().map_err(|_| ParsePointError)?,
            ))
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

struct RockIterator {
    point: Point,
    delta: Point,
    finish: Point,
}

impl RockIterator {
    fn from_points(start: Point, finish: Point) -> Self {
        let delta = Point((finish.0 - start.0).signum(), (finish.1 - start.1).signum());
        RockIterator {
            point: start - delta,
            delta,
            finish,
        }
    }
}

impl Iterator for RockIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.point == self.finish {
            None
        } else {
            self.point = self.point + self.delta;
            Some(self.point)
        }
    }
}

fn read_rocks(input: &str) -> HashSet<Point> {
    let mut rocks = HashSet::new();

    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .filter_map(|text| match text.parse::<Point>() {
                    Ok(pt) => Some(pt),
                    Err(_) => None,
                })
                .reduce(|start, finish| {
                    rocks.extend(RockIterator::from_points(start, finish));
                    finish
                })
        })
        .for_each(drop);

    rocks
}

const DOWN: Point = Point(0, 1);
const DOWN_LEFT: Point = Point(-1, 1);
const DOWN_RIGHT: Point = Point(1, 1);

fn apply_gravity(sand: Point, occupied: &HashSet<Point>, floor: Floor) -> Option<Point> {
    for direction in [DOWN, DOWN_LEFT, DOWN_RIGHT] {
        let consider = sand + direction;
        if !occupied.contains(&consider) && !floor.blocks(&consider) {
            return Some(consider);
        }
    }
    None
}

#[derive(Clone, Copy, PartialEq)]
enum FloorType {
    EndlessVoid,
    InfinitePlane,
}

#[derive(Clone, Copy, PartialEq)]
struct Floor {
    floor_type: FloorType,
    y: i32,
}

impl Floor {
    fn blocks(&self, pt: &Point) -> bool {
        match self.floor_type {
            FloorType::EndlessVoid => false,
            FloorType::InfinitePlane => pt.1 >= self.y,
        }
    }
}

fn resting_sand_quantity(input: &str, floor_type: FloorType) -> u32 {
    let mut resting_sand = 0;
    let mut occupied = read_rocks(input);
    let floor = Floor {
        floor_type,
        y: occupied.iter().map(|pt| pt.1).max().unwrap_or(0) + 2,
    };

    let mut sand = Point(500, 0);
    while sand.1 < floor.y {
        match apply_gravity(sand, &occupied, floor) {
            Some(point) => {
                sand = point;
            }
            None => {
                occupied.insert(sand);
                resting_sand += 1;
                if sand == Point(500, 0) {
                    break;
                }
                sand = Point(500, 0);
            }
        }
    }

    resting_sand
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(resting_sand_quantity(input, FloorType::EndlessVoid))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(resting_sand_quantity(input, FloorType::InfinitePlane))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_rock_iterator() {
        let mut rocks = RockIterator::from_points(Point(498, 4), Point(498, 6));
        assert_eq!(rocks.next(), Some(Point(498, 4)));
        assert_eq!(rocks.next(), Some(Point(498, 5)));
        assert_eq!(rocks.next(), Some(Point(498, 6)));
        assert_eq!(rocks.next(), None);
    }

    #[test]
    fn test_read_rocks() {
        let input = advent_of_code::read_file("examples", 14);
        let rocks = read_rocks(&input);
        assert_eq!(rocks.len(), 20);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
