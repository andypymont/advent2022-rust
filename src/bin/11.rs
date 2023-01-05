use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

impl Operation {
    fn apply(&self, item: u32) -> u32 {
        match self {
            Operation::Add(operand) => item + operand,
            Operation::Multiply(operand) => item * operand,
            Operation::Square => item * item,
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "new = old * old" {
            return Ok(Operation::Square);
        }

        let s = s.strip_prefix("new = old ").unwrap_or("");
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            Err(ParseOperationError)
        } else {
            let operand: u32 = parts[1].parse().map_err(|_| ParseOperationError)?;
            match parts[0] {
                "*" => Ok(Operation::Multiply(operand)),
                "+" => Ok(Operation::Add(operand)),
                _ => Err(ParseOperationError),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: usize,
    starting_items: Vec<u32>,
    operation: Operation,
    test: u32,
    throw_if_true: usize,
    throw_if_false: usize,
}

#[derive(Debug, PartialEq)]
struct ParseMonkeyError;

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id: Result<usize, Self::Err> = Err(ParseMonkeyError);
        let mut operation: Result<Operation, Self::Err> = Err(ParseMonkeyError);
        let mut test: Result<u32, Self::Err> = Err(ParseMonkeyError);
        let mut throw_if_true: Result<usize, Self::Err> = Err(ParseMonkeyError);
        let mut throw_if_false: Result<usize, Self::Err> = Err(ParseMonkeyError);
        let mut starting_items: Vec<u32> = Vec::new();

        for line in s.lines() {
            let line = line.trim();
            if line.starts_with("Monkey") {
                id = line
                    .strip_prefix("Monkey ")
                    .unwrap_or("")
                    .strip_suffix(':')
                    .unwrap_or("")
                    .parse()
                    .map_err(|_| ParseMonkeyError);
            } else if line.starts_with("Starting items") {
                for item_str in line
                    .strip_prefix("Starting items: ")
                    .unwrap_or("")
                    .split(", ")
                {
                    if let Ok(item) = item_str.parse::<u32>() {
                        starting_items.push(item);
                    }
                }
            } else if line.starts_with("Operation") {
                operation = line
                    .strip_prefix("Operation: ")
                    .unwrap_or("")
                    .parse()
                    .map_err(|_| ParseMonkeyError);
            } else if line.starts_with("Test") {
                test = line
                    .strip_prefix("Test: divisible by ")
                    .unwrap_or("")
                    .parse()
                    .map_err(|_| ParseMonkeyError);
            } else if line.starts_with("If true") {
                throw_if_true = line
                    .strip_prefix("If true: throw to monkey ")
                    .unwrap_or("")
                    .parse()
                    .map_err(|_| ParseMonkeyError);
            } else if line.starts_with("If false") {
                throw_if_false = line
                    .strip_prefix("If false: throw to monkey ")
                    .unwrap_or("")
                    .parse()
                    .map_err(|_| ParseMonkeyError);
            }
        }

        Ok(Monkey {
            id: id?,
            starting_items,
            operation: operation?,
            test: test?,
            throw_if_true: throw_if_true?,
            throw_if_false: throw_if_false?,
        })
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for section in input.split("\n\n") {
        if let Ok(monkey) = section.parse::<Monkey>() {
            monkeys.push(monkey);
        }
    }

    monkeys
}

fn monkey_business(monkeys: &Vec<Monkey>, rounds: u32) -> u32 {
    let mut items: HashMap<usize, VecDeque<u32>> = HashMap::new();
    let mut inspection_counts: HashMap<usize, u32> = HashMap::new();
    for monkey in monkeys {
        let mut inventory: VecDeque<u32> = VecDeque::new();
        inventory.extend(monkey.starting_items.iter());
        items.insert(monkey.id, inventory);
    }

    for _ in 0..rounds {
        for monkey in monkeys {
            // inspect and queue items for throwing
            let mut thrown: Vec<(usize, u32)> = Vec::new();
            items.entry(monkey.id).and_modify(|inventory| {
                while let Some(item) = inventory.pop_front() {
                    let item = monkey.operation.apply(item) / 3;
                    let target = if item % monkey.test == 0 {
                        monkey.throw_if_true
                    } else {
                        monkey.throw_if_false
                    };
                    thrown.push((target, item));
                }
            });

            // record the number of inspections
            let inspections = thrown.len() as u32;
            inspection_counts
                .entry(monkey.id)
                .and_modify(|i| *i += inspections)
                .or_insert(inspections);

            // throw items to other monkeys
            for (target, item) in thrown {
                items
                    .entry(target)
                    .and_modify(|inventory| inventory.push_back(item))
                    .or_default();
            }
        }
    }

    let (one, two): (u32, u32) =
        inspection_counts
            .values()
            .fold((0, 0), |(biggest, big), count| {
                if count > &biggest {
                    (*count, biggest)
                } else if count > &big {
                    (biggest, *count)
                } else {
                    (biggest, big)
                }
            });
    one * two
}

pub fn part_one(input: &str) -> Option<u32> {
    let monkeys = parse_monkeys(input);
    Some(monkey_business(&monkeys, 20))
}

pub fn part_two(_input: &str) -> Option<u32> {
    // let monkeys = parse_monkeys(input);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation_multiply() {
        assert_eq!("new = old * 19".parse(), Ok(Operation::Multiply(19)),);
    }

    #[test]
    fn test_parse_operation_add() {
        assert_eq!("new = old + 5".parse(), Ok(Operation::Add(5)),);
    }

    #[test]
    fn test_parse_operation_square() {
        assert_eq!("new = old * old".parse(), Ok(Operation::Square),);
    }

    #[test]
    fn test_parse_monkey() {
        let monkey_zero = Monkey {
            id: 0,
            starting_items: vec![79, 98],
            operation: Operation::Multiply(19),
            test: 23,
            throw_if_true: 2,
            throw_if_false: 3,
        };
        assert_eq!(
            concat![
                "Monkey 0:\n",
                "  Starting items: 79, 98\n",
                "  Operation: new = old * 19\n",
                "  Test: divisible by 23\n",
                "    If true: throw to monkey 2\n",
                "    If false: throw to monkey 3\n",
            ]
            .parse(),
            Ok(monkey_zero)
        );
    }

    #[test]
    fn test_parse_monkeys() {
        let input = advent_of_code::read_file("examples", 11);
        let result = parse_monkeys(&input);

        assert_eq!(
            result,
            vec![
                Monkey {
                    id: 0,
                    starting_items: vec![79, 98],
                    operation: Operation::Multiply(19),
                    test: 23,
                    throw_if_true: 2,
                    throw_if_false: 3,
                },
                Monkey {
                    id: 1,
                    starting_items: vec![54, 65, 75, 74],
                    operation: Operation::Add(6),
                    test: 19,
                    throw_if_true: 2,
                    throw_if_false: 0,
                },
                Monkey {
                    id: 2,
                    starting_items: vec![79, 60, 97],
                    operation: Operation::Square,
                    test: 13,
                    throw_if_true: 1,
                    throw_if_false: 3,
                },
                Monkey {
                    id: 3,
                    starting_items: vec![74],
                    operation: Operation::Add(3),
                    test: 17,
                    throw_if_true: 0,
                    throw_if_false: 1,
                },
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
