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
        rock_paper_scissors(a, strategy_to_hand(a, b))
    });
    println!("Following the strategy guide CORRECTLY we would get a score of {correct_score}");
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}
#[derive(Debug, Clone)]
struct HandInvalidErr;
impl Error for HandInvalidErr {}
impl fmt::Display for HandInvalidErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not convert to Hand")
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
            _ => Err(HandInvalidErr),
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

fn strategy_to_hand<'a>(opponent_hand: Hand, strategy: &'a str) -> Hand {
    match strategy {
        // X means we need to lose
        "X" => match opponent_hand {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
        // Y means we need to draw
        "Y" => opponent_hand.clone(),
        // Z means we need to win
        "Z" => match opponent_hand {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        },
        _ => panic!("Invalid strategy: {}", strategy),
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
