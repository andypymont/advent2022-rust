use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Signal {
    Integer(i32),
    List(Vec<Signal>),
}

impl Signal {
    fn to_list(&self) -> Vec<Self> {
        match self {
            Signal::List(l) => l.clone(),
            Signal::Integer(i) => vec![Signal::Integer(*i)],
        }
    }

    fn new_divider_packet(integer: i32) -> Self {
        Signal::List(vec![Signal::List(vec![Signal::Integer(integer)])])
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let (Self::Integer(a), Self::Integer(b)) = (self, other) {
            return Some(a.cmp(b));
        }

        let one = self.to_list();
        let two = other.to_list();
        let mut ix = 0;

        while ix < one.len() && ix < two.len() {
            let ord = one[ix].partial_cmp(&two[ix]);
            if ord.unwrap_or(Ordering::Equal) != Ordering::Equal {
                return ord;
            }
            ix += 1;
        }

        one.len().partial_cmp(&two.len())
    }
}

#[derive(Debug, PartialEq)]
struct ParseSignalError;

impl Signal {
    fn parse_list_from_chars(chars: &Vec<char>) -> Result<Self, ParseSignalError> {
        let mut brackets = 0;
        let mut pos = 0;
        let mut child = String::new();
        let mut children = Vec::new();

        while pos < chars.len() {
            let ch = chars[pos];
            if ch == '[' {
                if brackets > 0 {
                    child.push(ch);
                }
                brackets += 1;
            } else if ch == ']' {
                brackets -= 1;
                match brackets.cmp(&0) {
                    Ordering::Less => return Err(ParseSignalError),
                    Ordering::Equal => {
                        if !child.is_empty() {
                            children.push(child.parse::<Signal>()?);
                            break;
                        }
                    }
                    Ordering::Greater => child.push(ch),
                };
            } else if ch == ',' && brackets == 1 {
                children.push(child.parse::<Signal>()?);
                child = String::new();
            } else {
                child.push(ch);
            }
            pos += 1;
        }

        Ok(Signal::List(children))
    }

    fn parse_number_from_chars(chars: &Vec<char>) -> Result<Self, ParseSignalError> {
        let number: Result<i32, ParseSignalError> = String::from_iter(chars)
            .parse()
            .map_err(|_| ParseSignalError);
        Ok(Signal::Integer(number?))
    }
}

impl FromStr for Signal {
    type Err = ParseSignalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            Err(ParseSignalError)
        } else if chars[0] == '[' {
            Self::parse_list_from_chars(&chars)
        } else if chars[0].is_numeric() {
            Self::parse_number_from_chars(&chars)
        } else {
            Err(ParseSignalError)
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct SignalPair(Signal, Signal);

impl SignalPair {
    fn is_correctly_ordered(&self) -> bool {
        self.0 <= self.1
    }
}

impl FromStr for SignalPair {
    type Err = ParseSignalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.len() == 2 {
            let first = lines[0].parse()?;
            let second = lines[1].parse()?;
            Ok(SignalPair(first, second))
        } else {
            Err(ParseSignalError)
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input(input);

    Some(
        pairs
            .iter()
            .enumerate()
            .map(|(ix, pair)| {
                if pair.is_correctly_ordered() {
                    u32::try_from(ix + 1).unwrap_or(0)
                } else {
                    0
                }
            })
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let pairs = parse_input(input);

    let lower_divider = Signal::new_divider_packet(2);
    let upper_divider = Signal::new_divider_packet(6);

    let indices =
        pairs
            .iter()
            .flat_map(|p| [p.0.clone(), p.1.clone()])
            .fold((1, 2), |(low, mid), signal| {
                if signal <= lower_divider {
                    (low + 1, mid + 1)
                } else if signal <= upper_divider {
                    (low, mid + 1)
                } else {
                    (low, mid)
                }
            });

    Some(indices.0 * indices.1)
}

fn parse_input(input: &str) -> Vec<SignalPair> {
    input
        .split("\n\n")
        .filter_map(|section| match section.parse::<SignalPair>() {
            Err(_) => None,
            Ok(pair) => Some(pair),
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_signal_integer() {
        let input = "13";
        let chars = input.chars().collect();
        assert_eq!(
            Signal::parse_number_from_chars(&chars),
            Ok(Signal::Integer(13))
        );
    }

    #[test]
    fn test_parse_signal_empty_list() {
        let input = "[]";
        assert_eq!(input.parse(), Ok(Signal::List(vec![])));
    }

    #[test]
    fn test_parse_signal_list_of_integers() {
        let input = "[1,2,3]";
        assert_eq!(
            input.parse(),
            Ok(Signal::List(vec![
                Signal::Integer(1),
                Signal::Integer(2),
                Signal::Integer(3),
            ]))
        );
    }

    #[test]
    fn test_parse_signal_list_of_list_of_integers() {
        let input = "[[1,2],[3,4]]";
        assert_eq!(
            input.parse(),
            Ok(Signal::List(vec![
                Signal::List(vec![Signal::Integer(1), Signal::Integer(2),]),
                Signal::List(vec![Signal::Integer(3), Signal::Integer(4),])
            ]))
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
