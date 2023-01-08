use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point(i32, i32);

#[derive(Debug, PartialEq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() == 2 {
            let x = parts[0]
                .strip_prefix("x=")
                .unwrap_or("")
                .parse()
                .map_err(|_| ParsePointError)?;
            let y = parts[1]
                .strip_prefix("y=")
                .unwrap_or("")
                .parse()
                .map_err(|_| ParsePointError)?;
            Ok(Point(x, y))
        } else {
            Err(ParsePointError)
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range(i32, i32);

#[derive(Debug, PartialEq)]
struct Sensor {
    location: Point,
    closest_beacon: Point,
}

impl FromStr for Sensor {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() == 2 {
            let location = parts[0].strip_prefix("Sensor at ").unwrap_or("").parse()?;
            let closest_beacon = parts[1]
                .strip_prefix("closest beacon is at ")
                .unwrap_or("")
                .parse()?;
            Ok(Sensor {
                location,
                closest_beacon,
            })
        } else {
            Err(ParsePointError)
        }
    }
}

impl Sensor {
    fn beacon_distance(&self) -> i32 {
        (self.closest_beacon.0 - self.location.0).abs()
            + (self.closest_beacon.1 - self.location.1).abs()
    }

    fn covered_range_for_row(&self, row: i32) -> Range {
        let dist = self.beacon_distance() - (self.location.1 - row).abs();
        if dist < 0 {
            Range(self.location.0, self.location.0)
        } else {
            Range(self.location.0 - dist, self.location.0 + dist + 1)
        }
    }
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .filter_map(|line| match line.parse::<Sensor>() {
            Ok(sensor) => Some(sensor),
            Err(_) => None,
        })
        .collect()
}

fn non_beacon_positions(sensors: &[Sensor], row: i32) -> i32 {
    let mut ranges: Vec<Range> = sensors
        .iter()
        .map(|s| s.covered_range_for_row(row))
        .collect();
    ranges.sort();

    let mut x = ranges[0].0;
    let mut count: i32 = 0;
    for range in ranges {
        // if we already passed this range (fully overlapped by another), skip it
        if range.1 <= x {
            continue;
        }

        // skip any empty space between the previous x position and this range
        if range.0 > x {
            x = range.0
        }

        // add values from the current position up until the end of the range, then move to the end
        count += range.1 - x;
        x = range.1;
    }

    let beacons_in_row = {
        let positions: HashSet<i32> = sensors
            .iter()
            .filter_map(|s| {
                if s.closest_beacon.1 == row {
                    Some(s.closest_beacon.0)
                } else {
                    None
                }
            })
            .collect();
        positions.len() as i32
    };

    count - beacons_in_row
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(non_beacon_positions(&parse_sensors(input), 2_000_000))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sensor() {
        assert_eq!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".parse(),
            Ok(Sensor {
                location: Point(2, 18),
                closest_beacon: Point(-2, 15)
            })
        );
    }

    #[test]
    fn test_sensor_beacon_distance() {
        let sensor = Sensor {
            location: Point(9, 16),
            closest_beacon: Point(10, 18),
        };
        assert_eq!(sensor.beacon_distance(), 3);
    }

    #[test]
    fn test_sensor_covered_range_for_row() {
        let sensor = Sensor {
            location: Point(9, 16),
            closest_beacon: Point(10, 16),
        };
        assert_eq!(sensor.covered_range_for_row(16), Range(8, 11),);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        let sensors = parse_sensors(&input);
        assert_eq!(non_beacon_positions(&sensors, 10), 26);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
