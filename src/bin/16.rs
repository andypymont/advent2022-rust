use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ValveInfo {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct ParseValveSystemError;

impl FromStr for ValveInfo {
    type Err = ParseValveSystemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();
        if words.len() < 10 {
            Err(ParseValveSystemError)
        } else {
            let name = words[1].to_string();
            let flow_rate: i32 = words[4]
                .strip_prefix("rate=")
                .unwrap_or("")
                .strip_suffix(';')
                .unwrap_or("")
                .parse()
                .map_err(|_| ParseValveSystemError)?;
            let tunnels = words[9..]
                .iter()
                .map(|s| s.replace(',', ""))
                .collect();
            Ok(ValveInfo {
                name,
                flow_rate,
                tunnels,
            })
        }
    }
}

#[derive(Debug, Default)]
struct ValveSystem {
    flow_rates: HashMap<i32, i32>,
    graph: HashMap<i32, HashMap<i32, i32>>,
}

struct ValveSystemWalkState {
    time: i32,
    position: i32,
    open_valves: i32,
    pressure: i32,
}

impl ValveSystem {
    fn get_flow_rate(&self, valve_id: i32) -> i32 {
        *self.flow_rates.get(&valve_id).unwrap_or(&0)
    }

    fn best_pressure_possibilities(&self, minutes: i32) -> HashMap<i32, i32> {
        let mut results = HashMap::new();
        let mut consider = VecDeque::new();
        consider.push_back(ValveSystemWalkState {
            time: minutes,
            position: 0,
            open_valves: 0,
            pressure: 0,
        });

        while let Some(state) = consider.pop_front() {
            results
                .entry(state.open_valves)
                .and_modify(|current_best: &mut i32| {
                    *current_best = state.pressure.max(*current_best)
                })
                .or_insert(state.pressure);

            if let Some(node) = self.graph.get(&state.position) {
                for (neighbour, distance) in node.iter() {
                    if state.open_valves & neighbour == 0 {
                        let new_time = state.time - distance - 1;
                        if new_time >= 0 {
                            let extra_pressure = self.get_flow_rate(*neighbour) * new_time;
                            let new_state = ValveSystemWalkState {
                                time: new_time,
                                position: *neighbour,
                                open_valves: state.open_valves | neighbour,
                                pressure: state.pressure + extra_pressure,
                            };
                            consider.push_back(new_state);
                        }
                    }
                }
            }
        }

        results
    }

    fn best_pressure_possible(&self, minutes: i32, actors: usize) -> Option<i32> {
        let possibilities = self.best_pressure_possibilities(minutes);
        if actors == 1 {
            possibilities.values().max().copied()
        } else if actors == 2 {
            possibilities
                .iter()
                .flat_map(|(first_valves, first_pressure)| {
                    possibilities
                        .clone()
                        .iter()
                        .filter_map(move |(second_valves, second_pressure)| {
                            if first_valves & second_valves == 0 {
                                Some(first_pressure + second_pressure)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<i32>>()
                })
                .max()
        } else {
            None
        }
    }
}

impl FromStr for ValveSystem {
    type Err = ParseValveSystemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut names: HashMap<String, i32> = HashMap::new();
        let mut flow_rates: HashMap<String, i32> = HashMap::new();
        let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
        let mut next_valve_no: i32 = 1;

        for line in s.lines() {
            let valve: ValveInfo = line.parse()?;
            let number = if valve.name == "AA" {
                0
            } else if valve.flow_rate > 0 {
                let number = next_valve_no;
                next_valve_no *= 2;
                number
            } else {
                -1
            };
            connections
                .entry(valve.name.to_string())
                .or_default()
                .extend(valve.tunnels);
            if number == -1 {
                continue;
            } else {
                names.insert(valve.name.to_string(), number);
                flow_rates.insert(valve.name.to_string(), valve.flow_rate);
            }
        }

        let mut graph: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for (start_name, start_no) in names.clone() {
            let mut visited = HashSet::new();
            let mut consider = VecDeque::new();
            consider.push_back((start_name.to_string(), 0));
            while let Some((location, steps)) = consider.pop_front() {
                visited.insert(location.to_string());

                if location != start_name && location != "AA" {
                    if let Some(finish) = names.get(&location) {
                        graph
                            .entry(start_no)
                            .and_modify(|node: &mut HashMap<i32, i32>| {
                                node.insert(*finish, steps);
                            })
                            .or_insert_with(|| {
                                let mut node = HashMap::new();
                                node.insert(*finish, steps);
                                node
                            });
                    }
                }

                for adjacent in connections.entry(location.to_string()).or_default().iter() {
                    if !visited.contains(adjacent) {
                        consider.push_back((adjacent.to_string(), steps + 1))
                    }
                }
            }
        }

        let flow_rates = flow_rates
            .iter()
            .map(|(name, rate)| (*names.get(name).unwrap_or(&0), *rate))
            .collect::<HashMap<i32, i32>>();

        Ok(ValveSystem { flow_rates, graph })
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let system: ValveSystem = input.parse().unwrap_or_default();
    system.best_pressure_possible(30, 1)
}

pub fn part_two(input: &str) -> Option<i32> {
    let system: ValveSystem = input.parse().unwrap_or_default();
    system.best_pressure_possible(26, 2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valve_info() {
        assert_eq!(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".parse(),
            Ok(ValveInfo {
                name: "AA".to_string(),
                flow_rate: 0,
                tunnels: vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
            }),
        )
    }

    #[test]
    fn test_parse_valve_system() {
        let input = advent_of_code::read_file("examples", 16);

        let parsed = input.parse::<ValveSystem>();
        assert_eq!(parsed.is_err(), false);

        if let Ok(system) = parsed {
            assert_eq!(system.get_flow_rate(0), 0);
            assert_eq!(system.get_flow_rate(1), 13);
            assert_eq!(system.get_flow_rate(2), 2);
            assert_eq!(system.get_flow_rate(4), 20);
            assert_eq!(system.get_flow_rate(8), 3);
            assert_eq!(system.get_flow_rate(16), 22);
            assert_eq!(system.get_flow_rate(32), 21);
            assert_eq!(system.get_flow_rate(64), 0);

            if let Some(node) = system.graph.get(&0) {
                assert_eq!(node.len(), 6);
                assert_eq!(node.get(&1), Some(&1));
                assert_eq!(node.get(&2), Some(&2));
                assert_eq!(node.get(&4), Some(&1));
                assert_eq!(node.get(&8), Some(&2));
                assert_eq!(node.get(&16), Some(&5));
                assert_eq!(node.get(&32), Some(&2));
            }
        }
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
