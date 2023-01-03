use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn beats(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn beaten_by(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

#[derive(Debug, PartialEq, Eq)]
struct PartOneGame {
    opponent: Move,
    me: Move,
}

impl PartOneGame {
    fn outcome(&self) -> Outcome {
        if self.me == self.opponent {
            Outcome::Draw
        } else if self.opponent == self.me.beats() {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    fn score(&self) -> u32 {
        self.me.score() + self.outcome().score()
    }
}

impl FromStr for PartOneGame {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opp_str, me_str) = s.split_once(' ').ok_or(ParseGameError)?;

        let opponent = match opp_str {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(ParseGameError),
        }?;
        let me = match me_str {
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err(ParseGameError),
        }?;

        Ok(PartOneGame { me, opponent })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PartTwoGame {
    opponent: Move,
    outcome: Outcome,
}

impl PartTwoGame {
    fn me(&self) -> Move {
        match self.outcome {
            Outcome::Draw => self.opponent,
            Outcome::Loss => self.opponent.beats(),
            Outcome::Win => self.opponent.beaten_by(),
        }
    }
    
    fn score(&self) -> u32 {
        self.me().score() + self.outcome.score()
    }
}

impl FromStr for PartTwoGame {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opp_str, outcome_str) = s.split_once(' ').ok_or(ParseGameError)?;

        let opponent = match opp_str {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(ParseGameError),
        }?;
        let outcome = match outcome_str {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(ParseGameError),
        }?;

        Ok(PartTwoGame { opponent, outcome })
    }
}

fn parse_part_one_games(input: &str) -> Result<Vec<PartOneGame>, ParseGameError> {
    let mut games: Vec<PartOneGame> = Vec::new();

    for line in input.lines() {
        let game = line.parse::<PartOneGame>()?;
        games.push(game);
    }

    Ok(games)
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse_part_one_games(input);
    match parsed {
        Ok(games) => Some(games.iter().map(PartOneGame::score).sum()),
        Err(_) => None,
    }
}

fn parse_part_two_games(input: &str) -> Result<Vec<PartTwoGame>, ParseGameError> {
    let mut games: Vec<PartTwoGame> = Vec::new();

    for line in input.lines() {
        let game = line.parse::<PartTwoGame>()?;
        games.push(game);
    }

    Ok(games)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse_part_two_games(input);
    match parsed {
        Ok(games) => Some(games.iter().map(PartTwoGame::score).sum()),
        Err(_) => None,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_rock_scores_one() {
        assert_eq!(Move::Rock.score(), 1);
    }

    #[test]
    fn test_move_paper_scores_two() {
        assert_eq!(Move::Paper.score(), 2);
    }

    #[test]
    fn test_move_scissors_scores_three() {
        assert_eq!(Move::Scissors.score(), 3);
    }

    #[test]
    fn test_part_one_game_from_string_a_y() {
        let parsed: PartOneGame = "A Y".parse().unwrap();
        assert!(matches!(parsed.opponent, Move::Rock));
        assert!(matches!(parsed.me, Move::Paper));
    }

    #[test]
    fn test_part_one_game_from_string_b_x() {
        let parsed: PartOneGame = "B X".parse().unwrap();
        assert!(matches!(parsed.opponent, Move::Paper));
        assert!(matches!(parsed.me, Move::Rock));
    }

    #[test]
    fn test_score_part_one_game_rock_paper() {
        let game = PartOneGame { opponent: Move::Rock, me: Move::Paper };
        assert!(matches!(game.outcome(), Outcome::Win));
        assert_eq!(game.score(), 8);
    }

    #[test]
    fn test_score_part_one_game_paper_rock() {
        let game = PartOneGame { opponent: Move::Paper, me: Move::Rock };
        assert!(matches!(game.outcome(), Outcome::Loss));
        assert_eq!(game.score(), 1);
    }

    #[test]
    fn test_score_part_one_game_scissors_scissors() {
        let game = PartOneGame { opponent: Move::Scissors, me: Move::Scissors };
        assert!(matches!(game.outcome(), Outcome::Draw));
        assert_eq!(game.score(), 6);
    }

    #[test]
    fn test_parse_part_one_games() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(
            parse_part_one_games(&input),
            Ok(vec![
                PartOneGame { opponent: Move::Rock, me: Move::Paper },
                PartOneGame { opponent: Move::Paper, me: Move::Rock },
                PartOneGame { opponent: Move::Scissors, me: Move::Scissors },
            ])
        )
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two_game_from_string_a_y() {
        let parsed: PartTwoGame = "A Y".parse().unwrap();
        assert!(matches!(parsed.opponent, Move::Rock));
        assert!(matches!(parsed.outcome, Outcome::Draw));
    }

    #[test]
    fn test_part_two_game_from_string_b_x() {
        let parsed: PartTwoGame = "B X".parse().unwrap();
        assert!(matches!(parsed.opponent, Move::Paper));
        assert!(matches!(parsed.outcome, Outcome::Loss));
    }

    #[test]
    fn test_score_part_two_game_rock_draw() {
        let game = PartTwoGame { opponent: Move::Rock, outcome: Outcome::Draw };
        assert_eq!(game.score(), 4);
    }

    #[test]
    fn test_score_part_two_game_paper_loss() {
        let game = PartTwoGame { opponent: Move::Paper, outcome: Outcome::Loss };
        assert_eq!(game.score(), 1);
    }

    #[test]
    fn test_score_part_two_game_scissors_win() {
        let game = PartTwoGame { opponent: Move::Scissors, outcome: Outcome::Win };
        assert_eq!(game.score(), 7);
    }

    fn test_parse_part_two_games() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(
            parse_part_two_games(&input),
            Ok(vec![
                PartTwoGame { opponent: Move::Rock, outcome: Outcome::Draw },
                PartTwoGame { opponent: Move::Paper, outcome: Outcome::Loss },
                PartTwoGame { opponent: Move::Scissors, outcome: Outcome::Win },
            ])
        )
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
