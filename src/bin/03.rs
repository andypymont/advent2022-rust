use std::collections::HashSet;

fn priority(item: Option<&char>) -> u32 {
    match item {
        None => 0,
        Some(c) => {
            if c.is_lowercase() {
                (*c as u32) - 96
            } else {
                (*c as u32) - 38
            }
        }
    }
}

fn backpack_priority(backpack: &str) -> u32 {
    let mut compartment_one: HashSet<char> = HashSet::new();
    let mut compartment_two: HashSet<char> = HashSet::new();

    let middle = backpack.len() / 2;

    for (pos, char) in backpack.chars().enumerate() {
        if pos < middle {
            compartment_one.insert(char);
        } else {
            compartment_two.insert(char);
        }
    }

    let mut both_compartments = compartment_one.intersection(&compartment_two);
    priority(both_compartments.next())
}

fn group_badge_priority(first: &str, second: &str, third: &str) -> u32 {
    let first_set = first.chars().collect::<HashSet<char>>();
    let second_set = second.chars().collect::<HashSet<char>>();
    let third_set = third.chars().collect::<HashSet<char>>();
    priority(
        first_set
            .intersection(&second_set)
            .find(|item| third_set.contains(item)),
    )
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let total: u32 = input.lines().map(backpack_priority).sum();
    Some(total)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut backpacks = input.lines().peekable();

    while backpacks.peek().is_some() {
        let first = backpacks.next().unwrap_or("");
        let second = backpacks.next().unwrap_or("");
        let third = backpacks.next().unwrap_or("");
        total += group_badge_priority(first, second, third);
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_backpack() {
        assert_eq!(backpack_priority("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    }

    #[test]
    fn test_second_backpack() {
        assert_eq!(backpack_priority("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
    }

    #[test]
    fn test_third_backpack() {
        assert_eq!(backpack_priority("PmmdzqPrVvPwwTWBwg"), 42);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_first_group() {
        assert_eq!(
            group_badge_priority(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            18
        );
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
