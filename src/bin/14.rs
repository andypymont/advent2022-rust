const GRID_COLS: usize = 700;
const GRID_ROWS: usize = 700;

struct ParsePointError;

fn read_point(text: &str) -> Result<(usize, usize), ParsePointError> {
    let parts: Vec<&str> = text.split(',').collect();
    if parts.len() == 2 {
        let x: usize = parts[0].parse().map_err(|_| ParsePointError)?;
        let y: usize = parts[1].parse().map_err(|_| ParsePointError)?;
        Ok((x, y))
    } else {
        Err(ParsePointError)
    }
}

fn read_input(input: &str) -> Vec<bool> {
    let mut rocks = vec![false; GRID_COLS * GRID_ROWS];

    for line in input.lines() {
        line.split(" -> ")
            .filter_map(|text| match read_point(text) {
                Ok(pt) => Some(pt),
                Err(_) => None,
            })
            .reduce(|(ax, ay), (bx, by)| {
                if ax == bx {
                    let min_y = ay.min(by);
                    let max_y = ay.max(by);
                    for y in min_y..=max_y {
                        rocks[(y * GRID_COLS) + ax] = true;
                    }
                } else if ay == by {
                    let min_x = ax.min(bx);
                    let max_x = ax.max(bx);
                    for x in min_x..=max_x {
                        rocks[(ay * GRID_COLS) + x] = true;
                    }
                }
                (bx, by)
            });
    }

    rocks
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut occupied = read_input(input);
    let mut rocks = 0;
    let maximum = {
        if let Some(last_rock) = occupied.iter().rposition(|v| *v) {
            let x = last_rock % GRID_COLS;
            last_rock - x + (GRID_COLS * 2)
        } else {
            0
        }
    };

    let mut rock = 500;
    while rock <= maximum {
        let down = rock + GRID_COLS;
        let left = down - 1;
        let right = down + 1;

        rock = match (occupied[down], occupied[left], occupied[right]) {
            (true, true, true) => {
                occupied[rock] = true;
                rocks += 1;
                500
            }
            (false, _, _) => down,
            (true, false, _) => left,
            (true, true, false) => right,
        }
    }

    Some(rocks)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let mut occupied = read_input(input);
    let mut rocks = 0;
    let maximum = {
        if let Some(last_rock) = occupied.iter().rposition(|v| *v) {
            let x = last_rock % GRID_COLS;
            last_rock - x + (GRID_COLS * 2)
        } else {
            0
        }
    };

    let mut rock = 500;
    while !occupied[500] {
        let down = rock + GRID_COLS;
        let left = down - 1;
        let right = down + 1;

        rock = match (
            occupied[down] || down >= maximum,
            occupied[left] || left >= maximum,
            occupied[right] || right >= maximum,
        ) {
            (true, true, true) => {
                occupied[rock] = true;
                rocks += 1;
                500
            }
            (false, _, _) => down,
            (true, false, _) => left,
            (true, true, false) => right,
        }
    }

    Some(rocks)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = advent_of_code::read_file("examples", 14);
        let rocks = read_input(&input);

        assert_eq!(rocks.iter().map(|x| u32::from(*x)).sum::<u32>(), 20);
        assert_eq!(rocks[0], false);
        assert_eq!(rocks[(4 * GRID_COLS) + 498], true);
        assert_eq!(rocks[(4 * GRID_COLS) + 500], false);
        assert_eq!(rocks[(4 * GRID_COLS) + 502], true);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
