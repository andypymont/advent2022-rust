use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const UP: Point = Point { x: 0, y: -1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const DOWN: Point = Point { x: 0, y: 1 };

fn compass() -> Vec<Point> {
    vec![UP, RIGHT, DOWN, LEFT]
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TreeInfo {
    location: Point,
    visible: bool,
    scenic_score: u32,
}

impl TreeInfo {
    fn from_tree(location: Point, forest: &HashMap<Point, u32>) -> TreeInfo {
        let mut visible = false;
        let mut scenic_score = 1;
        let height = forest.get(&location).unwrap_or(&0);

        for direction in compass() {
            let mut target = location;
            let mut distance: u32 = 0;

            loop {
                target = target + direction;
                if !forest.contains_key(&target) {
                    visible = true;
                    break;
                }
                distance += 1;
                if forest.get(&target).unwrap_or(&0) >= height {
                    break;
                }
            }
            scenic_score *= distance;
        }

        TreeInfo {
            location,
            visible,
            scenic_score,
        }
    }
}

fn read_forest(input: &str) -> HashMap<Point, u32> {
    let mut forest = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pt = Point {
                x: x as i32,
                y: y as i32,
            };
            let height = ch.to_digit(10).unwrap_or(0);
            forest.insert(pt, height);
        }
    }

    forest
}

fn trees_in_forest(forest: &HashMap<Point, u32>) -> HashSet<TreeInfo> {
    let mut trees = HashSet::new();

    for location in forest.keys() {
        let info = TreeInfo::from_tree(*location, forest);
        trees.insert(info);
    }

    trees
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest = read_forest(input);
    let trees = trees_in_forest(&forest);

    Some(trees.iter().map(|tree| u32::from(tree.visible)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = read_forest(input);
    let trees = trees_in_forest(&forest);

    trees.iter().map(|tree| tree.scenic_score).max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_forest() {
        let input = advent_of_code::read_file("examples", 8);
        let forest = read_forest(&input);

        assert_eq!(forest.len(), 25);
        assert_eq!(forest.get(&Point { x: 0, y: 0 }), Some(&3));
        assert_eq!(forest.get(&Point { x: 3, y: 0 }), Some(&7));
        assert_eq!(forest.get(&Point { x: 1, y: 2 }), Some(&5));
        assert_eq!(forest.get(&Point { x: 6, y: 2 }), None);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
