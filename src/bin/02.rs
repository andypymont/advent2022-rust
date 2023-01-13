fn part_one_score(game: &str) -> u32 {
    match game {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0,
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let total = input.lines().map(part_one_score).sum();
    Some(total)
}

fn part_two_score(game: &str) -> u32 {
    match game {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let total = input.lines().map(part_two_score).sum();
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_score_a_y() {
        assert_eq!(part_one_score("A Y"), 8);
    }

    #[test]
    fn test_part_one_score_b_x() {
        assert_eq!(part_one_score("B X"), 1);
    }

    #[test]
    fn test_part_one_score_c_z() {
        assert_eq!(part_one_score("C Z"), 6);
    }

    #[test]
    fn test_part_one_score_d_w() {
        assert_eq!(part_one_score("D W"), 0);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two_score_a_y() {
        assert_eq!(part_two_score("A Y"), 4);
    }

    #[test]
    fn test_part_two_score_b_x() {
        assert_eq!(part_two_score("B X"), 1);
    }

    #[test]
    fn test_part_two_score_c_z() {
        assert_eq!(part_two_score("C Z"), 7);
    }

    #[test]
    fn test_part_two_score_d_w() {
        assert_eq!(part_two_score("D W"), 0);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
