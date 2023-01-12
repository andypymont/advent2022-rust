use std::collections::VecDeque;
use std::num::ParseIntError;

fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        match line.parse::<i32>() {
            Err(e) => return Err(e),
            Ok(i) => parsed.push(i),
        }
    }
    Ok(parsed)
}

fn mix(list: Vec<i32>) -> Vec<i32> {
    let mut circle = VecDeque::new();
    circle.extend(list.iter().enumerate());

    for each_ix in 0..list.len() {
        while let Some((ix, value)) = circle.pop_front() {
            if ix == each_ix {
                let dist = value.rem_euclid(circle.len() as i32) as usize;
                circle.rotate_left(dist);
                circle.push_back((ix, value));
                break;
            }
            circle.push_back((ix, value));
        }
    }

    circle.iter().map(|(_i, v)| **v).collect()
}

fn grove_coordinates(list: Vec<i32>) -> i32 {
    let zero = list.iter().position(|value| *value == 0).unwrap_or(0);
    [1000, 2000, 3000]
        .iter()
        .map(|ix| list[(zero + ix) % list.len()])
        .sum()
}

pub fn part_one(input: &str) -> Option<i32> {
    match parse_input(input) {
        Err(_) => None,
        Ok(list) => Some(grove_coordinates(mix(list))),
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(parse_input(&input), Ok(vec![1, 2, -3, 3, -2, 0, 4]),);
    }

    #[test]
    fn test_mix() {
        let list = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(mix(list), vec![0, 3, -2, 1, 2, -3, 4],);
    }

    #[test]
    fn test_grove_coordinates() {
        let list = vec![3, -2, 1, 2, -3, 4, 0];
        assert_eq!(grove_coordinates(list), 3);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
