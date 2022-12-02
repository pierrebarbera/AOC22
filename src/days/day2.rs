use io;
use std::path::Path;

pub fn day2(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day2, which is a valid file path.");
    }

    let incorrect_score = run_strategy(&args[0], rock_paper_scissors);

    println!("Following the strategy guide INCORRECTLY we would get a score of {incorrect_score}");

    let correct_score = run_strategy(&args[0], |a, b| {
        rock_paper_scissors(a, strategy_to_hand(a, b))
    });
    println!("Following the strategy guide CORRECTLY we would get a score of {correct_score}");
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
                if let Some(hands) = line.split_once(' ') {
                    score += score_func(hands.0, hands.1);
                }
            }
        }
    }
    return score;
}

fn strategy_to_hand<'a>(opponent_hand: &'a str, strategy: &'a str) -> &'a str {
    match strategy {
        // X means we need to lose
        "X" => match opponent_hand {
            "A" => "C",
            "B" => "A",
            "C" => "B",
            _ => panic!("Invalid hand: {}", opponent_hand),
        },
        // Y means we need to draw
        "Y" => opponent_hand,
        // Z means we need to win
        "Z" => match opponent_hand {
            "A" => "B",
            "B" => "C",
            "C" => "A",
            _ => panic!("Invalid hand: {}", opponent_hand),
        },
        _ => panic!("Invalid strategy: {}", strategy),
    }
}

fn rock_paper_scissors(opponent_hand: &str, player_hand: &str) -> u32 {
    let op_idx = hand_index(opponent_hand);
    let pl_idx = hand_index(player_hand);
    return SCORE_MATRIX[pl_idx][op_idx];
}

const SCORE_MATRIX: [[u32; 3]; 3] = [[4, 1, 7], [8, 5, 2], [3, 9, 6]];

fn hand_index(hand: &str) -> usize {
    match hand {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("Invalid hand: {}", hand),
    }
}
