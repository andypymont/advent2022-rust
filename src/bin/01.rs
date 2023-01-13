#[must_use]
pub fn read_totals_from_input(input: &str) -> Vec<u32> {
    let mut elves: Vec<u32> = Vec::new();

    for carried in input.split("\n\n") {
        let total = carried
            .trim()
            .split('\n')
            .map(|s| s.parse().unwrap_or(0))
            .sum();
        elves.push(total);
    }

    elves
}

#[must_use]
pub fn max_total_calories(calories_by_elf: &[u32], quantity: usize) -> Vec<u32> {
    let mut totals = calories_by_elf.to_vec();
    totals.sort_unstable();
    totals.reverse();
    totals.truncate(quantity);
    totals
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let totals = read_totals_from_input(input);
    Some(max_total_calories(&totals, 1).iter().sum())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let totals = read_totals_from_input(input);
    Some(max_total_calories(&totals, 3).iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_totals_from_input() {
        let input = advent_of_code::read_file("examples", 1);
        let totals = read_totals_from_input(&input);
        assert_eq!(totals, vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn test_max_total_calories() {
        let totals = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(max_total_calories(&totals, 3), vec![24000, 11000, 10000]);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
