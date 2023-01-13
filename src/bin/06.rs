use std::collections::HashMap;

fn marker_location(input: &str, distinct_chars: usize) -> Option<u32> {
    let mut last_seen: HashMap<char, usize> = HashMap::new();

    for (pos, ch) in input.chars().enumerate() {
        last_seen.insert(ch, pos);

        if pos >= distinct_chars {
            let purge_earlier_than = 1 + pos - distinct_chars;
            last_seen.retain(|_, old_pos| *old_pos >= purge_earlier_than);
        }
        if last_seen.len() == distinct_chars {
            return Some((pos + 1) as u32);
        }
    }

    None
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    marker_location(input, 4)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    marker_location(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
