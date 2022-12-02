use std::fs::read_to_string;

enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}
use crate::Move::*;
use crate::Outcome::*;

impl Move {
    fn wins_against(&self, other: &Move) -> Outcome {
        match (self, &other) {
            (Rock, Rock) => Draw,
            (Paper, Paper) => Draw,
            (Scissors, Scissors) => Draw,
            (Rock, Scissors) => Win,
            (Scissors, Paper) => Win,
            (Paper, Rock) => Win,
            (Scissors, Rock) => Lose,
            (Paper, Scissors) => Lose,
            (Rock, Paper) => Lose,
        }
    }

    fn find_move_that_gives_result_against_this(&self, outcome: &Outcome) -> Move {
        vec![Rock, Paper, Scissors]
            .into_iter()
            .find(|other_move| {
                let outcome_with_this = other_move.wins_against(self);
                *outcome == outcome_with_this
            })
            .expect("BUG, couldn't find move that gave outcome")
    }
}

fn parse_opponent_move(s: &str) -> Move {
    match s {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Couldn't understand your move: {}", s),
    }
}

// For part 1
fn parse_your_move(s: &str) -> Move {
    match s {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("Couldn't understand your move: {}", s),
    }
}

// For part 2
fn parse_intended_result(s: &str) -> Outcome {
    match s {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!("Couldn't understand your move: {}", s),
    }
}

fn get_score(your_move: &Move, opponent_move: &Move) -> usize {
    let outcome = your_move.wins_against(opponent_move);

    let move_score = match your_move {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let win_score = match outcome {
        Lose => 0,
        Draw => 3,
        Win => 6,
    };

    move_score + win_score
}

fn main() {
    let s = read_to_string("input").expect("failed to read input file");

    let lines: Vec<&str> = s.split('\n').filter(|s| !s.is_empty()).collect();

    // Part 1
    let moves: Vec<(Move, Move)> = lines
        .iter()
        .map(|s| {
            let words: Vec<&str> = s.split(' ').collect();
            let opponent_move = parse_opponent_move(words[0]);
            let your_move = parse_your_move(words[1]);
            (opponent_move, your_move)
        })
        .collect();

    let score: usize = moves
        .iter()
        .map(|(opponent_move, your_move)| get_score(your_move, opponent_move))
        .sum();

    println!("{}", score);

    // Part 2
    let moves: Vec<(Move, Move)> = lines
        .iter()
        .map(|s| {
            let words: Vec<&str> = s.split(' ').collect();
            let opponent_move = parse_opponent_move(words[0]);
            let intended_outcome = parse_intended_result(words[1]);
            let your_move =
                opponent_move.find_move_that_gives_result_against_this(&intended_outcome);
            (opponent_move, your_move)
        })
        .collect();

    let score: usize = moves
        .iter()
        .map(|(opponent_move, your_move)| get_score(your_move, opponent_move))
        .sum();

    println!("{}", score);
}
