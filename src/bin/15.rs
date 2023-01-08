use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    fn within_bounds(&self, min_coord: i32, max_coord: i32) -> bool {
        self.0 >= min_coord && self.0 <= max_coord && self.1 >= min_coord && self.1 <= max_coord
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
        self.location.manhattan_distance(&self.closest_beacon)
    }

    fn covered_range_for_row(&self, row: i32) -> Range {
        let dist = self.beacon_distance() - (self.location.1 - row).abs();
        if dist < 0 {
            Range(self.location.0, self.location.0)
        } else {
            Range(self.location.0 - dist, self.location.0 + dist + 1)
        }
    }

    fn positions_just_outside_range(&self) -> SensorExteriorPositionIterator {
        SensorExteriorPositionIterator::from_sensor(self)
    }
}

struct SensorExteriorPositionIterator {
    sensor_location: Point,
    distance: i32,
    position: i32,
}

impl SensorExteriorPositionIterator {
    fn from_sensor(sensor: &Sensor) -> Self {
        SensorExteriorPositionIterator {
            sensor_location: sensor.location,
            distance: sensor.beacon_distance() + 1,
            position: 0,
        }
    }
}

impl Iterator for SensorExteriorPositionIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let phase = self.position / self.distance;
        let phase_pos = self.position % self.distance;

        let point: Option<Point> = match phase {
            0 => Some(Point(
                self.sensor_location.0 + phase_pos,
                self.sensor_location.1 + self.distance - phase_pos,
            )),
            1 => Some(Point(
                self.sensor_location.0 + self.distance - phase_pos,
                self.sensor_location.1 - phase_pos,
            )),
            2 => Some(Point(
                self.sensor_location.0 - phase_pos,
                self.sensor_location.1 - self.distance + phase_pos,
            )),
            3 => Some(Point(
                self.sensor_location.0 - self.distance + phase_pos,
                self.sensor_location.1 + phase_pos,
            )),
            _ => None,
        };

        if point.is_some() {
            self.position += 1;
            point
        } else {
            None
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

fn beacon_position(sensors: &[Sensor], min_coord: i32, max_coord: i32) -> Option<Point> {
    for sensor in sensors {
        for position in sensor
            .positions_just_outside_range()
            .filter(|pos| pos.within_bounds(min_coord, max_coord))
        {
            if !sensors.iter().any(|sensor| {
                sensor.location.manhattan_distance(&position) <= sensor.beacon_distance()
            }) {
                return Some(position);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(non_beacon_positions(&parse_sensors(input), 2_000_000))
}

pub fn part_two(input: &str) -> Option<i64> {
    let result = beacon_position(&parse_sensors(input), 0, 4_000_000);
    match result {
        None => None,
        Some(beacon) => {
            let x = (beacon.0 as i64) * 4_000_000;
            let y = beacon.1 as i64;
            Some(x + y)
        }
    }
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
    fn test_sensor_exterior_position_iterator() {
        let sensor = Sensor {
            location: Point(0, 0),
            closest_beacon: Point(2, 0),
        };
        let exterior: HashSet<Point> = sensor.positions_just_outside_range().collect();
        assert_eq!(exterior.len(), 12);
        assert_eq!(exterior.contains(&Point(1, 2)), true);
        assert_eq!(exterior.contains(&Point(2, 1)), true);
        assert_eq!(exterior.contains(&Point(3, 0)), true);
        assert_eq!(exterior.contains(&Point(2, -1)), true);
        assert_eq!(exterior.contains(&Point(1, -2)), true);
        assert_eq!(exterior.contains(&Point(0, -3)), true);
        assert_eq!(exterior.contains(&Point(-1, -2)), true);
        assert_eq!(exterior.contains(&Point(-2, -1)), true);
        assert_eq!(exterior.contains(&Point(-3, 0)), true);
        assert_eq!(exterior.contains(&Point(-2, 1)), true);
        assert_eq!(exterior.contains(&Point(-1, 2)), true);
        assert_eq!(exterior.contains(&Point(0, 3)), true);
        assert_eq!(exterior.contains(&Point(1, 1)), false);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        let sensors = parse_sensors(&input);
        assert_eq!(beacon_position(&sensors, 0, 20), Some(Point(14, 11)));
    }
}
