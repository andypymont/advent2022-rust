const SNAFU_BASE: i64 = 5;

fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(pos, digit)| {
            if let Ok(pos) = u32::try_from(pos) {
                let base = SNAFU_BASE.pow(pos);
                match digit {
                    '2' => Some(2),
                    '1' => Some(1),
                    '0' => Some(0),
                    '-' => Some(-1),
                    '=' => Some(-2),
                    _ => None,
                }
                .map(|qty| base * qty)
            } else {
                None
            }
        })
        .sum()
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut snafu = String::new();
    let mut remaining = decimal;

    loop {
        let ix = usize::try_from((remaining + 2) % 5).unwrap_or(0);
        snafu.push(['=', '-', '0', '1', '2'][ix]);
        remaining = (remaining + 2) / 5;
        if remaining == 0 {
            break;
        }
    }

    snafu.chars().rev().collect()
}

#[must_use]
pub fn part_one(input: &str) -> Option<String> {
    Some(decimal_to_snafu(input.lines().map(snafu_to_decimal).sum()))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1"), 1);
        assert_eq!(snafu_to_decimal("2"), 2);
        assert_eq!(snafu_to_decimal("1="), 3);
        assert_eq!(snafu_to_decimal("1-"), 4);
        assert_eq!(snafu_to_decimal("10"), 5);
        assert_eq!(snafu_to_decimal("11"), 6);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("2="), 8);
        assert_eq!(snafu_to_decimal("2-"), 9);
        assert_eq!(snafu_to_decimal("20"), 10);
        assert_eq!(snafu_to_decimal("1=0"), 15);
        assert_eq!(snafu_to_decimal("1-0"), 20);
        assert_eq!(snafu_to_decimal("1=11-2"), 2022);
        assert_eq!(snafu_to_decimal("1-0---0"), 12_345);
        assert_eq!(snafu_to_decimal("1121-1110-1=0"), 3_14_159_265);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(1), "1".to_string());
        assert_eq!(decimal_to_snafu(2), "2".to_string());
        assert_eq!(decimal_to_snafu(4), "1-".to_string());
        assert_eq!(decimal_to_snafu(7), "12".to_string());
        assert_eq!(decimal_to_snafu(15), "1=0".to_string());
        assert_eq!(decimal_to_snafu(2022), "1=11-2".to_string());
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }
}
