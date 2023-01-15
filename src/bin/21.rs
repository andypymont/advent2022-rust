use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Multiply => a * b,
            Self::Divide => a / b,
        }
    }

    fn inputs_to_get_value(&self, current_a: i64, current_b: i64, target: i64) -> (i64, i64) {
        match self {
            Self::Add => (target - current_b, target - current_a),
            Self::Subtract => (target + current_b, current_a - target),
            Self::Divide => (target * current_b, current_a / target),
            Self::Multiply => (target / current_b, target / current_a),
        }
    }
}

struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err(ParseOperationError),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Monkey {
    Value(i64),
    Calculation(String, Operation, String),
}

impl Monkey {
    fn value(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Self::Value(val) => *val,
            Self::Calculation(a, op, b) => {
                let a = monkeys.get(a).unwrap_or(&Monkey::Value(0));
                let b = monkeys.get(b).unwrap_or(&Monkey::Value(0));
                op.apply(a.value(monkeys), b.value(monkeys))
            }
        }
    }

    fn inputs_to_get_value(
        &self,
        monkeys: &HashMap<String, Monkey>,
        target: i64,
    ) -> Option<(i64, i64)> {
        match self {
            Monkey::Value(_) => None,
            Monkey::Calculation(a, op, b) => {
                let a = monkeys.get(a).unwrap_or(&Monkey::Value(0));
                let b = monkeys.get(b).unwrap_or(&Monkey::Value(0));
                Some(op.inputs_to_get_value(a.value(monkeys), b.value(monkeys), target))
            }
        }
    }
}

fn parse_monkeys(input: &str) -> HashMap<String, Monkey> {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let name = parts[0].to_string();
            let words: Vec<&str> = parts[1].split(' ').collect();
            if words.len() == 1 {
                let value = words[0].parse().unwrap_or(0);
                monkeys.insert(name, Monkey::Value(value));
            } else if words.len() == 3 {
                let a = words[0].to_string();
                let b = words[2].to_string();
                let op: Operation = words[1].parse().unwrap_or(Operation::Add);
                monkeys.insert(name, Monkey::Calculation(a, op, b));
            }
        }
    }

    monkeys
}

#[must_use]
pub fn part_one(input: &str) -> Option<i64> {
    let monkeys = parse_monkeys(input);
    monkeys.get("root").map(|monkey| monkey.value(&monkeys))
}

#[must_use]
pub fn part_two(input: &str) -> Option<i64> {
    let mut monkeys = parse_monkeys(input);

    if let Some(Monkey::Calculation(a, _, b)) = monkeys.get("root") {
        let mut queue = VecDeque::new();
        monkeys.insert(
            "root".to_string(),
            Monkey::Calculation(a.to_string(), Operation::Subtract, b.to_string()),
        );
        queue.push_front(("root", 0));

        while let Some((name, expected)) = queue.pop_front() {
            if name == "humn" {
                return Some(expected);
            }

            let monkey = monkeys.get(name).unwrap_or(&Monkey::Value(0));
            if let Monkey::Calculation(a, _, b) = monkey {
                if let Some((expect_a, expect_b)) = monkey.inputs_to_get_value(&monkeys, expected) {
                    queue.push_back((a, expect_a));
                    queue.push_back((b, expect_b));
                }
            }
        }

        None
    } else {
        None
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkeys() {
        let input = advent_of_code::read_file("examples", 21);
        let monkeys = parse_monkeys(&input);
        assert_eq!(monkeys.len(), 15);
        assert_eq!(
            monkeys.get("root"),
            Some(&Monkey::Calculation(
                "pppw".to_string(),
                Operation::Add,
                "sjmn".to_string()
            ))
        );
    }

    #[test]
    fn test_monkey_value() {
        let monkeys = HashMap::new();
        assert_eq!(Monkey::Value(4).value(&monkeys), 4);
        assert_eq!(Monkey::Value(27).value(&monkeys), 27);
        assert_eq!(Monkey::Value(-5).value(&monkeys), -5);
    }

    #[test]
    fn test_monkey_calculation() {
        let mut monkeys = HashMap::new();

        monkeys.insert("pppw".to_string(), Monkey::Value(9));
        monkeys.insert("sjmn".to_string(), Monkey::Value(3));

        let add = Monkey::Calculation("pppw".to_string(), Operation::Add, "sjmn".to_string());
        let sub = Monkey::Calculation("pppw".to_string(), Operation::Subtract, "sjmn".to_string());
        let div = Monkey::Calculation("pppw".to_string(), Operation::Divide, "sjmn".to_string());
        let mul = Monkey::Calculation("pppw".to_string(), Operation::Multiply, "sjmn".to_string());

        assert_eq!(add.value(&monkeys), 12);
        assert_eq!(sub.value(&monkeys), 6);
        assert_eq!(div.value(&monkeys), 3);
        assert_eq!(mul.value(&monkeys), 27);
    }

    #[test]
    fn test_monkey_inputs_to_get_value() {
        let mut monkeys = HashMap::new();

        monkeys.insert("pppw".to_string(), Monkey::Value(9));
        monkeys.insert("sjmn".to_string(), Monkey::Value(3));

        let add = Monkey::Calculation("pppw".to_string(), Operation::Add, "sjmn".to_string());
        let sub = Monkey::Calculation("pppw".to_string(), Operation::Subtract, "sjmn".to_string());
        let div = Monkey::Calculation("pppw".to_string(), Operation::Divide, "sjmn".to_string());
        let mul = Monkey::Calculation("pppw".to_string(), Operation::Multiply, "sjmn".to_string());
        let val = Monkey::Value(4);

        assert_eq!(add.inputs_to_get_value(&monkeys, 6), Some((3, -3)));
        assert_eq!(sub.inputs_to_get_value(&monkeys, 10), Some((13, -1)));
        assert_eq!(div.inputs_to_get_value(&monkeys, 9), Some((27, 1)));
        assert_eq!(mul.inputs_to_get_value(&monkeys, 54), Some((18, 6)));
        assert_eq!(val.inputs_to_get_value(&monkeys, 18), None);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
