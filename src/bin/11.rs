use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum WorryManagementStrategy {
    DivideByThree,
    Modulo(u64),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn apply(&self, item: u64, strategy: &WorryManagementStrategy) -> u64 {
        let value = match self {
            Operation::Add(operand) => item + operand,
            Operation::Multiply(operand) => item * operand,
            Operation::Square => item * item,
        };
        match strategy {
            WorryManagementStrategy::DivideByThree => value / 3,
            WorryManagementStrategy::Modulo(m) => value % m,
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
        if parts.len() == 2 {
            let operand: u64 = parts[1].parse().map_err(|_| ParseOperationError)?;
            match parts.first() {
                Some(&"*") => Ok(Operation::Multiply(operand)),
                Some(&"+") => Ok(Operation::Add(operand)),
                _ => Err(ParseOperationError),
            }
        } else {
            Err(ParseOperationError)
        }
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: usize,
    starting_items: Vec<u64>,
    operation: Operation,
    test: u64,
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
        let mut test: Result<u64, Self::Err> = Err(ParseMonkeyError);
        let mut throw_if_true: Result<usize, Self::Err> = Err(ParseMonkeyError);
        let mut throw_if_false: Result<usize, Self::Err> = Err(ParseMonkeyError);
        let mut starting_items: Vec<u64> = Vec::new();

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
                    if let Ok(item) = item_str.parse::<u64>() {
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

fn monkey_business(monkeys: &Vec<Monkey>, rounds: u64, part_two: bool) -> u64 {
    let mut items: HashMap<usize, VecDeque<u64>> = HashMap::new();
    let mut inspection_counts: HashMap<usize, u64> = HashMap::new();
    let mut mod_prod = 1;
    for monkey in monkeys {
        let mut inventory: VecDeque<u64> = VecDeque::new();
        inventory.extend(monkey.starting_items.iter());
        items.insert(monkey.id, inventory);
        mod_prod *= monkey.test;
    }

    let strategy = if part_two {
        WorryManagementStrategy::Modulo(mod_prod)
    } else {
        WorryManagementStrategy::DivideByThree
    };

    for _ in 0..rounds {
        for monkey in monkeys {
            // inspect and queue items for throwing
            let mut thrown: Vec<(usize, u64)> = Vec::new();
            items.entry(monkey.id).and_modify(|inventory| {
                while let Some(item) = inventory.pop_front() {
                    let item = monkey.operation.apply(item, &strategy);
                    let target = if item % monkey.test == 0 {
                        monkey.throw_if_true
                    } else {
                        monkey.throw_if_false
                    };
                    thrown.push((target, item));
                }
            });

            // record the number of inspections
            let inspections = thrown.len() as u64;
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

    let (one, two): (u64, u64) =
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

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = parse_monkeys(input);
    Some(monkey_business(&monkeys, 20, false))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = parse_monkeys(input);
    Some(monkey_business(&monkeys, 10_000, true))
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
        assert_eq!(part_one(&input), Some(10_605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2_713_310_158));
    }
}
