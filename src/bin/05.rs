use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Action {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq)]
struct ParseActionError;

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();

        if words.len() != 6 {
            Err(ParseActionError)
        } else {
            let quantity: usize = words[1].parse().map_err(|_| ParseActionError)?;
            let from: usize = words[3].parse().map_err(|_| ParseActionError)?;
            let to: usize = words[5].parse().map_err(|_| ParseActionError)?;
            Ok(Action { quantity, from, to })
        }
    }
}

fn parse_input(input: &str) -> (Vec<String>, Vec<Action>) {
    let mut stacks = Vec::new();
    let mut actions = Vec::new();

    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() != 2 {
        return (stacks, actions);
    }

    for line in parts[0].split('\n') {
        for (ix, pos) in (1..line.len()).step_by(4).enumerate() {
            let ch = line.chars().nth(pos).unwrap_or(' ');
            if stacks.len() <= ix {
                stacks.push(String::new());
            }
            if ch.is_alphabetic() {
                stacks[ix].push(ch);
            }
        }
    }

    for line in parts[1].split('\n') {
        if let Ok(action) = line.parse::<Action>() {
            actions.push(action)
        };
    }

    (stacks, actions)
}

fn do_action(stacks: Vec<String>, action: Action, flip_moved: bool) -> Vec<String> {
    let moved: String = {
        let picked_up = stacks[action.from - 1][..action.quantity].chars();
        if flip_moved {
            picked_up.rev().collect()
        } else {
            picked_up.collect()
        }
    };

    let mut result = Vec::new();
    for (ix, stack) in stacks.iter().enumerate() {
        let stack = {
            if ix + 1 == action.from {
                stack[action.quantity..].to_string()
            } else if ix + 1 == action.to {
                moved.clone() + stack
            } else {
                stack.clone()
            }
        };
        result.push(stack)
    }

    result
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, actions) = parse_input(input);

    for action in actions {
        stacks = do_action(stacks, action, true);
    }

    Some(
        stacks
            .iter()
            .map(|stack| stack.chars().next().unwrap_or(' '))
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, actions) = parse_input(input);

    for action in actions {
        stacks = do_action(stacks, action, false);
    }

    Some(
        stacks
            .iter()
            .map(|stack| stack.chars().next().unwrap_or(' '))
            .collect(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 5);
        let (stacks, moves) = parse_input(&input);

        assert_eq!(stacks, vec!["NZ", "DCM", "P"]);
        assert_eq!(
            moves,
            vec![
                Action {
                    quantity: 1,
                    from: 2,
                    to: 1
                },
                Action {
                    quantity: 3,
                    from: 1,
                    to: 3
                },
                Action {
                    quantity: 2,
                    from: 2,
                    to: 1
                },
                Action {
                    quantity: 1,
                    from: 1,
                    to: 2
                },
            ]
        );
    }

    #[test]
    fn test_first_action() {
        let before = vec!["NZ".to_string(), "DCM".to_string(), "P".to_string()];
        let action = Action {
            quantity: 1,
            from: 2,
            to: 1,
        };
        assert_eq!(do_action(before, action, true), vec!["DNZ", "CM", "P"],);
    }

    #[test]
    fn test_second_action() {
        let before = vec!["DNZ".to_string(), "CM".to_string(), "P".to_string()];
        let action = Action {
            quantity: 3,
            from: 1,
            to: 3,
        };
        assert_eq!(do_action(before, action, true), vec!["", "CM", "ZNDP"],);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
