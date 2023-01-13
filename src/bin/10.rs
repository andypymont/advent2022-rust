use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ParseInstructionError;

#[derive(Debug, PartialEq)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::NoOp)
        } else {
            let parts: Vec<&str> = s.split(' ').collect();
            if parts.len() != 2 || parts[0] != "addx" {
                Err(ParseInstructionError)
            } else {
                let x: i32 = parts[1].parse().map_err(|_| ParseInstructionError)?;
                Ok(Instruction::AddX(x))
            }
        }
    }
}

fn read_program(input: &str) -> Result<Vec<Instruction>, ParseInstructionError> {
    let mut program = Vec::new();

    for line in input.lines() {
        match line.parse::<Instruction>() {
            Err(e) => return Err(e),
            Ok(instruction) => program.push(instruction),
        };
    }

    Ok(program)
}

fn run_program(program: Vec<Instruction>) -> Vec<i32> {
    let mut x: i32 = 1;
    let mut values = Vec::new();
    values.push(x);

    for instruction in program {
        values.push(x);
        match instruction {
            Instruction::NoOp => {}
            Instruction::AddX(add) => {
                x += add;
                values.push(x);
            }
        };
    }

    values
}

fn signal_strength(program_results: &[i32]) -> i32 {
    (20..=220)
        .step_by(40)
        .map(|cycle| {
            let register_value = program_results[cycle - 1];
            register_value * i32::try_from(cycle).unwrap_or(0)
        })
        .sum()
}

fn crt_image(program_results: &[i32]) -> String {
    let mut image = String::new();
    let mut line = String::new();

    for (ix, x) in program_results.iter().enumerate() {
        let pixel = i32::try_from(ix % 40).unwrap_or(0);
        line.push(if *x - 1 == pixel || *x == pixel || *x + 1 == pixel {
            '#'
        } else {
            '.'
        });
        if line.len() == 40 {
            image.push_str(&line);
            image.push('\n');
            line = String::new();
        };
    }

    image
}

#[must_use]
pub fn part_one(input: &str) -> Option<i32> {
    match read_program(input) {
        Err(_) => None,
        Ok(program) => Some(signal_strength(&run_program(program))),
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<String> {
    match read_program(input) {
        Err(_) => None,
        Ok(program) => Some(crt_image(&run_program(program))),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_instruction_noop() {
        assert_eq!("noop".parse::<Instruction>(), Ok(Instruction::NoOp),);
    }

    #[test]
    fn test_read_instruction_addx() {
        assert_eq!("addx 3".parse::<Instruction>(), Ok(Instruction::AddX(3)),);
    }

    #[test]
    fn test_read_instruction_addx_negative() {
        assert_eq!("addx -5".parse::<Instruction>(), Ok(Instruction::AddX(-5)),);
    }

    #[test]
    fn test_read_instruction_error() {
        assert_eq!("subx 4".parse::<Instruction>(), Err(ParseInstructionError),)
    }

    #[test]
    fn test_read_program() {
        assert_eq!(
            read_program("noop\naddx 3\naddx -5"),
            Ok(vec![
                Instruction::NoOp,
                Instruction::AddX(3),
                Instruction::AddX(-5)
            ])
        );
    }

    #[test]
    fn test_run_program() {
        let program: Vec<Instruction> = vec![
            Instruction::NoOp,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ];
        assert_eq!(run_program(program), vec![1, 1, 1, 4, 4, -1,]);
    }

    #[test]
    fn test_run_program_longer_example() {
        let input = advent_of_code::read_file("examples", 10);
        let program = read_program(&input).unwrap_or(Vec::new());
        let results = run_program(program);

        assert_eq!(results[19], 21);
        assert_eq!(results[59], 19);
        assert_eq!(results[99], 18);
        assert_eq!(results[139], 21);
        assert_eq!(results[179], 16);
        assert_eq!(results[219], 18);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let image = concat![
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######.....\n",
        ]
        .to_string();
        assert_eq!(part_two(&input), Some(image));
    }
}
