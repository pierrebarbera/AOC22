use io;
use std::error::Error;
use std::fmt;
use std::path::Path;

pub fn day2(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day2, which is a valid file path.");
    }

    let incorrect_score = run_strategy(&args[0], |a, b| {
        rock_paper_scissors(Hand::from_str(a).unwrap(), Hand::from_str(b).unwrap())
    });

    println!("Following the strategy guide INCORRECTLY we would get a score of {incorrect_score}");

    let correct_score = run_strategy(&args[0], |a, b| {
        let a = Hand::from_str(a).unwrap();
        let b = Strategy::from_str(b).unwrap();
        rock_paper_scissors(a, strategy_to_hand(a, b))
    });
    println!("Following the strategy guide CORRECTLY we would get a score of {correct_score}");
}
#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}
#[derive(Debug, Clone)]
struct HandInvalidErr {
    msg: String,
}
impl Error for HandInvalidErr {}
impl fmt::Display for HandInvalidErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not convert to Hand : {}", self.msg)
    }
}
impl Hand {
    fn as_str(&self) -> &'static str {
        match self {
            Hand::Rock => "A",
            Hand::Paper => "B",
            Hand::Scissors => "C",
        }
    }
    fn from_str(input: &str) -> Result<Hand, HandInvalidErr> {
        match input {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(HandInvalidErr {
                msg: input.to_string(),
            }),
        }
    }
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn run_strategy<P, F>(filename: P, score_func: F) -> u32
where
    P: AsRef<Path>,
    F: Fn(&str, &str) -> u32,
{
    let mut score: u32 = 0;

    if let Ok(lines) = io::read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if let Some((op_hand, strat)) = line.split_once(' ') {
                    score += score_func(op_hand, strat);
                }
            }
        }
    }
    return score;
}

#[derive(Debug, Clone, Copy)]
enum Strategy {
    Lose = 0,
    Draw = 1,
    Win = 2,
}
#[derive(Debug, Clone)]
struct StrategyInvalidErr {
    msg: String,
}
impl Error for StrategyInvalidErr {}
impl fmt::Display for StrategyInvalidErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not convert to Strategy: {}", self.msg)
    }
}
impl Strategy {
    fn as_str(&self) -> &'static str {
        match self {
            Strategy::Lose => "X",
            Strategy::Draw => "Y",
            Strategy::Win => "Z",
        }
    }
    fn from_str(input: &str) -> Result<Strategy, StrategyInvalidErr> {
        match input {
            "X" => Ok(Strategy::Lose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            _ => Err(StrategyInvalidErr {
                msg: input.to_string(),
            }),
        }
    }
}
impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn strategy_to_hand(opponent_hand: Hand, strategy: Strategy) -> Hand {
    match strategy {
        // X means we need to lose
        Strategy::Lose => match opponent_hand {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
        // Y means we need to draw
        Strategy::Draw => opponent_hand.clone(),
        // Z means we need to win
        Strategy::Win => match opponent_hand {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        },
    }
}

fn rock_paper_scissors(opponent_hand: Hand, player_hand: Hand) -> u32 {
    let op_idx = hand_index(&opponent_hand);
    let pl_idx = hand_index(&player_hand);
    return SCORE_MATRIX[pl_idx][op_idx];
}

const SCORE_MATRIX: [[u32; 3]; 3] = [[4, 1, 7], [8, 5, 2], [3, 9, 6]];

fn hand_index(hand: &Hand) -> usize {
    match hand {
        Hand::Rock => 0,
        Hand::Paper => 1,
        Hand::Scissors => 2,
    }
}
