use std::collections::{HashSet, VecDeque};
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cube(i32, i32, i32);

impl Cube {
    fn neighbours(self) -> HashSet<Cube> {
        let mut neighbours = HashSet::new();
        neighbours.insert(self + Cube(1, 0, 0));
        neighbours.insert(self + Cube(-1, 0, 0));
        neighbours.insert(self + Cube(0, 1, 0));
        neighbours.insert(self + Cube(0, -1, 0));
        neighbours.insert(self + Cube(0, 0, 1));
        neighbours.insert(self + Cube(0, 0, -1));
        neighbours
    }

    fn within_bounds(&self, min_coord: i32, max_coord: i32) -> bool {
        self.0 >= min_coord
            && self.0 <= max_coord
            && self.1 >= min_coord
            && self.1 <= max_coord
            && self.2 >= min_coord
            && self.2 <= max_coord
    }
}

impl Add for Cube {
    type Output = Self;

    fn add(self, other: Cube) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Debug, PartialEq)]
struct ParseCubeError;

impl FromStr for Cube {
    type Err = ParseCubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 3 {
            let x = parts[0].parse().map_err(|_| ParseCubeError)?;
            let y = parts[1].parse().map_err(|_| ParseCubeError)?;
            let z = parts[2].parse().map_err(|_| ParseCubeError)?;
            Ok(Cube(x, y, z))
        } else {
            Err(ParseCubeError)
        }
    }
}

fn surface_area(cubes: &HashSet<Cube>) -> u32 {
    cubes
        .iter()
        .map(|cube| cube.neighbours().difference(cubes).fold(0, |a, _b| a + 1))
        .sum()
}

fn bounds(cubes: &HashSet<Cube>) -> (i32, i32) {
    let (min, max) = cubes
        .iter()
        .map(|cube| {
            (
                cube.0.min(cube.1).min(cube.2),
                cube.0.max(cube.1).max(cube.2),
            )
        })
        .fold((0, 0), |acc, cube| (acc.0.min(cube.0), acc.1.max(cube.1)));
    (min - 1, max + 1)
}

fn external_surface_area(cubes: &HashSet<Cube>) -> u32 {
    let (min_coord, max_coord) = bounds(cubes);

    let mut area = 0;
    let mut visited = HashSet::new();
    let mut consider = VecDeque::new();
    consider.push_back(Cube(min_coord, min_coord, min_coord));

    while let Some(location) = consider.pop_front() {
        if visited.contains(&location) {
            continue;
        }

        if cubes.contains(&location) {
            area += 1;
        } else {
            visited.insert(location);
            for neighbour in location.neighbours() {
                if neighbour.within_bounds(min_coord, max_coord) {
                    consider.push_back(neighbour);
                }
            }
        }
    }

    area
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut cubes = HashSet::new();
    for line in input.lines() {
        match line.parse::<Cube>() {
            Err(_) => return None,
            Ok(cube) => cubes.insert(cube),
        };
    }
    Some(surface_area(&cubes))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let mut cubes = HashSet::new();
    for line in input.lines() {
        match line.parse::<Cube>() {
            Err(_) => return None,
            Ok(cube) => cubes.insert(cube),
        };
    }
    Some(external_surface_area(&cubes))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cube() {
        assert_eq!("1,2,3".parse(), Ok(Cube(1, 2, 3)),);
        assert_eq!("1,-1,5".parse(), Ok(Cube(1, -1, 5)),);
    }

    #[test]
    fn test_cube_neighbours() {
        let cube = Cube(1, 2, 3);
        let neighbours = cube.neighbours();
        assert_eq!(neighbours.len(), 6);
        assert_eq!(neighbours.contains(&Cube(2, 2, 3)), true);
        assert_eq!(neighbours.contains(&Cube(1, 3, 3)), true);
        assert_eq!(neighbours.contains(&Cube(1, 2, 5)), false);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
