use std::collections::VecDeque;
use std::num::ParseIntError;

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        match line.parse::<i64>() {
            Err(e) => return Err(e),
            Ok(i) => parsed.push(i),
        }
    }
    Ok(parsed)
}

const DECRYPTION_KEY: i64 = 811_589_153;

fn apply_key(list: &[i64]) -> Vec<i64> {
    list.iter().map(|value| value * DECRYPTION_KEY).collect()
}

fn mix(list: &[i64], rounds: usize) -> Vec<i64> {
    let mut circle = VecDeque::new();
    circle.extend(list.iter().enumerate());

    for _ in 0..rounds {
        for ix in 0..list.len() {
            let pos = circle.iter().position(|i| i.0 == ix).unwrap_or(0);
            circle.rotate_left(pos);
            if let Some((ix, value)) = circle.pop_front() {
                let length = i64::try_from(circle.len()).unwrap_or(0);
                let distance = usize::try_from(value.rem_euclid(length)).unwrap_or(0);
                circle.rotate_left(distance);
                circle.push_back((ix, value));
            }
        }
    }

    circle.iter().map(|(_i, v)| **v).collect()
}

fn grove_coordinates(list: &[i64]) -> i64 {
    let zero = list.iter().position(|value| *value == 0).unwrap_or(0);
    [1000, 2000, 3000]
        .iter()
        .map(|ix| list[(zero + ix) % list.len()])
        .sum()
}

#[must_use]
pub fn part_one(input: &str) -> Option<i64> {
    match parse_input(input) {
        Err(_) => None,
        Ok(list) => Some(grove_coordinates(&mix(&list, 1))),
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<i64> {
    match parse_input(input) {
        Err(_) => None,
        Ok(list) => Some(grove_coordinates(&mix(&apply_key(&list), 10))),
    }
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
        assert_eq!(mix(&list, 1), vec![0, 3, -2, 1, 2, -3, 4]);
    }

    #[test]
    fn test_grove_coordinates() {
        let list = vec![3, -2, 1, 2, -3, 4, 0];
        assert_eq!(grove_coordinates(&list), 3);
    }

    #[test]
    fn test_apply_key() {
        let list = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(
            apply_key(&list),
            vec![
                811589153,
                1623178306,
                -2434767459,
                2434767459,
                -1623178306,
                0,
                3246356612
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
