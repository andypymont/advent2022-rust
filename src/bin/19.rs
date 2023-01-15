use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cost(u32, u32, u32);

impl Cost {
    fn max(&self, other: &Cost) -> Self {
        Cost(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }
}

#[derive(Debug, PartialEq)]
struct Blueprint {
    number: u32,
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

#[derive(Debug, PartialEq)]
struct ParseBlueprintError;

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number: Result<u32, ParseBlueprintError> = Err(ParseBlueprintError);
        let mut ore_robot_cost: Result<Cost, ParseBlueprintError> = Err(ParseBlueprintError);
        let mut clay_robot_cost: Result<Cost, ParseBlueprintError> = Err(ParseBlueprintError);
        let mut obsidian_robot_cost: Result<Cost, ParseBlueprintError> = Err(ParseBlueprintError);
        let mut geode_robot_cost: Result<Cost, ParseBlueprintError> = Err(ParseBlueprintError);

        for part in s.split(": ") {
            if part.starts_with("Blueprint") {
                number = part
                    .replace("Blueprint ", "")
                    .parse()
                    .map_err(|_| ParseBlueprintError);
            } else {
                for sentence in part.split(". ") {
                    if sentence.starts_with("Each ore robot") {
                        ore_robot_cost = match sentence
                            .replace("Each ore robot costs ", "")
                            .replace(" ore", "")
                            .parse::<u32>()
                        {
                            Ok(value) => Ok(Cost(value, 0, 0)),
                            Err(_) => Err(ParseBlueprintError),
                        }
                    } else if sentence.starts_with("Each clay robot") {
                        clay_robot_cost = match sentence
                            .replace("Each clay robot costs ", "")
                            .replace(" ore", "")
                            .parse::<u32>()
                        {
                            Ok(value) => Ok(Cost(value, 0, 0)),
                            Err(_) => Err(ParseBlueprintError),
                        }
                    } else if sentence.starts_with("Each obsidian robot") {
                        let sentence = sentence
                            .replace("Each obsidian robot costs ", "")
                            .replace(" clay", "");
                        let cost_parts: Vec<&str> = sentence.split(" ore and ").collect();
                        obsidian_robot_cost = if cost_parts.len() == 2 {
                            let ore = cost_parts[0].parse::<u32>();
                            let clay = cost_parts[1].parse::<u32>();
                            match (ore, clay) {
                                (Ok(ore), Ok(clay)) => Ok(Cost(ore, clay, 0)),
                                _ => Err(ParseBlueprintError),
                            }
                        } else {
                            Err(ParseBlueprintError)
                        }
                    } else if sentence.starts_with("Each geode robot") {
                        let sentence = sentence
                            .replace("Each geode robot costs ", "")
                            .replace(" obsidian.", "");
                        let cost_parts: Vec<&str> = sentence.split(" ore and ").collect();
                        geode_robot_cost = if cost_parts.len() == 2 {
                            let ore = cost_parts[0].parse::<u32>();
                            let obsidian = cost_parts[1].parse::<u32>();
                            match (ore, obsidian) {
                                (Ok(ore), Ok(obsidian)) => Ok(Cost(ore, 0, obsidian)),
                                _ => Err(ParseBlueprintError),
                            }
                        } else {
                            Err(ParseBlueprintError)
                        }
                    }
                }
            }
        }

        Ok(Self {
            number: number?,
            ore_robot_cost: ore_robot_cost?,
            clay_robot_cost: clay_robot_cost?,
            obsidian_robot_cost: obsidian_robot_cost?,
            geode_robot_cost: geode_robot_cost?,
        })
    }
}

impl Blueprint {
    fn most_robots_needed(&self) -> Cost {
        Cost(0, 0, 0)
            .max(&self.ore_robot_cost)
            .max(&self.clay_robot_cost)
            .max(&self.obsidian_robot_cost)
            .max(&self.geode_robot_cost)
    }

    fn most_geodes_openable(&self, minutes: u32) -> u32 {
        let most_robots_needed = self.most_robots_needed();
        let mut best = 0;
        let mut consider = VecDeque::new();
        consider.push_front(State::create_initial(minutes));

        while let Some(state) = consider.pop_front() {
            // DFS
            best = best.max(state.open_geodes);

            if state.maximum_achievable_open_geodes() < best {
                continue;
            }

            consider.extend(state.possible_moves(self, most_robots_needed));
        }

        best
    }
}

#[derive(Debug, PartialEq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    fn cost(&self, blueprint: &Blueprint) -> Cost {
        match self {
            Robot::Ore => blueprint.ore_robot_cost,
            Robot::Clay => blueprint.clay_robot_cost,
            Robot::Obsidian => blueprint.obsidian_robot_cost,
            Robot::Geode => blueprint.geode_robot_cost,
        }
    }
}

const ROBOT_TYPES: [Robot; 4] = [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode];

fn div_ceil(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct State {
    time: u32,
    open_geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
}

impl State {
    fn create_initial(time: u32) -> Self {
        Self {
            time,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
        }
    }

    fn maximum_achievable_open_geodes(&self) -> u32 {
        self.open_geodes + ((self.time * (self.time - 1)) / 2)
    }

    fn possible_moves(&self, blueprint: &Blueprint, most_robots_needed: Cost) -> HashSet<State> {
        let mut possible = HashSet::new();

        for robot in ROBOT_TYPES {
            let cost = robot.cost(blueprint);
            let mut minutes_until_start: u32 = 0;

            if match robot {
                Robot::Ore => self.ore_robots >= most_robots_needed.0,
                Robot::Clay => self.clay_robots >= most_robots_needed.1,
                Robot::Obsidian => self.obsidian_robots >= most_robots_needed.2,
                Robot::Geode => false,
            } {
                continue;
            }

            let ore_needed = cost.0.saturating_sub(self.ore);
            if ore_needed > 0 {
                if self.ore_robots == 0 {
                    continue;
                }
                minutes_until_start =
                    minutes_until_start.max(div_ceil(ore_needed, self.ore_robots));
            }

            let clay_needed = cost.1.saturating_sub(self.clay);
            if clay_needed > 0 {
                if self.clay_robots == 0 {
                    continue;
                }
                minutes_until_start =
                    minutes_until_start.max(div_ceil(clay_needed, self.clay_robots));
            }

            let obsidian_needed = cost.2.saturating_sub(self.obsidian);
            if obsidian_needed > 0 {
                if self.obsidian_robots == 0 {
                    continue;
                }
                minutes_until_start =
                    minutes_until_start.max(div_ceil(obsidian_needed, self.obsidian_robots));
            }

            let time = self.time.saturating_sub(1 + minutes_until_start);
            if time > 0 {
                let new_geodes = match robot {
                    Robot::Geode => time,
                    _ => 0,
                };
                possible.insert(State {
                    time,
                    open_geodes: self.open_geodes + new_geodes,
                    ore_robots: self.ore_robots + u32::from(robot == Robot::Ore),
                    clay_robots: self.clay_robots + u32::from(robot == Robot::Clay),
                    obsidian_robots: self.obsidian_robots + u32::from(robot == Robot::Obsidian),
                    ore: self.ore + (self.ore_robots * (minutes_until_start + 1)) - cost.0,
                    clay: self.clay + (self.clay_robots * (minutes_until_start + 1)) - cost.1,
                    obsidian: self.obsidian + (self.obsidian_robots * (minutes_until_start + 1))
                        - cost.2,
                });
            }
        }

        possible
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| match line.parse::<Blueprint>() {
                Err(_) => None,
                Ok(blueprint) => Some(blueprint.number * blueprint.most_geodes_openable(24)),
            })
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| match line.parse::<Blueprint>() {
                Err(_) => None,
                Ok(blueprint) => {
                    if blueprint.number <= 3 {
                        Some(blueprint.most_geodes_openable(32))
                    } else {
                        None
                    }
                }
            })
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blueprint() {
        let input = concat![
            "Blueprint 1: Each ore robot costs 4 ore. ",
            "Each clay robot costs 2 ore. ",
            "Each obsidian robot costs 3 ore and 14 clay. ",
            "Each geode robot costs 2 ore and 7 obsidian.",
        ];
        assert_eq!(
            input.parse(),
            Ok(Blueprint {
                number: 1,
                ore_robot_cost: Cost(4, 0, 0),
                clay_robot_cost: Cost(2, 0, 0),
                obsidian_robot_cost: Cost(3, 14, 0),
                geode_robot_cost: Cost(2, 0, 7),
            }),
        )
    }

    #[test]
    fn test_most_robots_needed() {
        let blueprint = Blueprint {
            number: 1,
            ore_robot_cost: Cost(4, 0, 0),
            clay_robot_cost: Cost(2, 0, 0),
            obsidian_robot_cost: Cost(3, 14, 0),
            geode_robot_cost: Cost(2, 0, 7),
        };
        assert_eq!(blueprint.most_robots_needed(), Cost(4, 14, 7));
    }

    #[test]
    fn test_possible_moves_initial() {
        let blueprint = Blueprint {
            number: 1,
            ore_robot_cost: Cost(4, 0, 0),
            clay_robot_cost: Cost(2, 0, 0),
            obsidian_robot_cost: Cost(3, 14, 0),
            geode_robot_cost: Cost(2, 0, 7),
        };
        let state = State {
            time: 24,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
        };
        let next_state_ore = State {
            time: 19,
            open_geodes: 0,
            ore_robots: 2,
            clay_robots: 0,
            obsidian_robots: 0,
            ore: 1,
            clay: 0,
            obsidian: 0,
        };
        let next_state_clay = State {
            time: 21,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 1,
            obsidian_robots: 0,
            ore: 1,
            clay: 0,
            obsidian: 0,
        };

        let possible = state.possible_moves(&blueprint, blueprint.most_robots_needed());
        assert_eq!(possible.len(), 2);
        assert_eq!(possible.contains(&next_state_ore), true);
        assert_eq!(possible.contains(&next_state_clay), true);
    }

    #[test]
    fn test_possible_moves_example_path() {
        let blueprint = Blueprint {
            number: 1,
            ore_robot_cost: Cost(4, 0, 0),
            clay_robot_cost: Cost(2, 0, 0),
            obsidian_robot_cost: Cost(3, 14, 0),
            geode_robot_cost: Cost(2, 0, 7),
        };
        let initial = State {
            time: 24,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
        };
        let one = State {
            time: 21,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 1,
            obsidian_robots: 0,
            ore: 1,
            clay: 0,
            obsidian: 0,
        };
        let two = State {
            time: 19,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 2,
            obsidian_robots: 0,
            ore: 1,
            clay: 2,
            obsidian: 0,
        };
        let three = State {
            time: 17,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 3,
            obsidian_robots: 0,
            ore: 1,
            clay: 6,
            obsidian: 0,
        };
        let four = State {
            time: 13,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 3,
            obsidian_robots: 1,
            ore: 2,
            clay: 4,
            obsidian: 0,
        };
        let five = State {
            time: 12,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 4,
            obsidian_robots: 1,
            ore: 1,
            clay: 7,
            obsidian: 1,
        };
        let six = State {
            time: 9,
            open_geodes: 0,
            ore_robots: 1,
            clay_robots: 4,
            obsidian_robots: 2,
            ore: 1,
            clay: 5,
            obsidian: 4,
        };
        let seven = State {
            time: 6,
            open_geodes: 6,
            ore_robots: 1,
            clay_robots: 4,
            obsidian_robots: 2,
            ore: 2,
            clay: 17,
            obsidian: 3,
        };
        let eight = State {
            time: 3,
            open_geodes: 9,
            ore_robots: 1,
            clay_robots: 4,
            obsidian_robots: 2,
            ore: 3,
            clay: 29,
            obsidian: 2,
        };
        let most_robots_needed = blueprint.most_robots_needed();
        assert_eq!(
            initial
                .possible_moves(&blueprint, most_robots_needed)
                .contains(&one),
            true
        );
        assert_eq!(
            one.possible_moves(&blueprint, most_robots_needed)
                .contains(&two),
            true
        );
        assert_eq!(
            two.possible_moves(&blueprint, most_robots_needed)
                .contains(&three),
            true
        );
        assert_eq!(
            three
                .possible_moves(&blueprint, most_robots_needed)
                .contains(&four),
            true
        );
        assert_eq!(
            four.possible_moves(&blueprint, most_robots_needed)
                .contains(&five),
            true
        );
        assert_eq!(
            five.possible_moves(&blueprint, most_robots_needed)
                .contains(&six),
            true
        );
        assert_eq!(
            six.possible_moves(&blueprint, most_robots_needed)
                .contains(&seven),
            true
        );
        assert_eq!(
            seven
                .possible_moves(&blueprint, most_robots_needed)
                .contains(&eight),
            true
        );
    }

    #[test]
    fn test_most_geodes_openable() {
        let blueprint = Blueprint {
            number: 1,
            ore_robot_cost: Cost(4, 0, 0),
            clay_robot_cost: Cost(2, 0, 0),
            obsidian_robot_cost: Cost(3, 14, 0),
            geode_robot_cost: Cost(2, 0, 7),
        };
        assert_eq!(blueprint.most_geodes_openable(24), 9);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(3472));
    }
}
