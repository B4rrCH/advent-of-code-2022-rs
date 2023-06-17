use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Strategy {
    Answer,
    Outcome,
}

trait RPS {
    fn get_better(worse_move: Move) -> Move;
    fn get_worse(better_move: Move) -> Move;
}

impl RPS for Move {
    fn get_better(worse_move: Move) -> Move {
        match worse_move {
            Move::Paper => Move::Scissors,
            Move::Rock => Move::Paper,
            Move::Scissors => Move::Rock,
        }
    }

    fn get_worse(better_move: Move) -> Move {
        match better_move {
            Move::Scissors => Move::Paper,
            Move::Paper => Move::Rock,
            Move::Rock => Move::Scissors,
        }
    }
}

impl std::cmp::PartialOrd<Move> for Move {
    fn partial_cmp(&self, other: &Move) -> Option<std::cmp::Ordering> {
        if *self == *other {
            Some(std::cmp::Ordering::Equal)
        } else {
            Some(match (*self, *other) {
                (Move::Rock, Move::Scissors) => std::cmp::Ordering::Greater,
                (Move::Scissors, Move::Paper) => std::cmp::Ordering::Greater,
                (Move::Paper, Move::Rock) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Less,
            })
        }
    }
}

fn parse_answer(c: char) -> Option<Move> {
    match c {
        'X' => Some(Move::Rock),
        'Y' => Some(Move::Paper),
        'Z' => Some(Move::Scissors),
        _ => None,
    }
}

fn get_outcome(c: char, opponent: Option<Move>) -> Option<Move> {
    match c {
        'X' => opponent.map(Move::get_worse),
        'Y' => opponent,
        'Z' => opponent.map(Move::get_better),
        _ => None,
    }
}

fn parse(s: String, strategy: Strategy) -> Option<(Move, Move)> {
    let opponent = match s.chars().nth(0) {
        Some('A') => Some(Move::Rock),
        Some('B') => Some(Move::Paper),
        Some('C') => Some(Move::Scissors),
        _ => None,
    };

    let player = match strategy {
        Strategy::Answer => s.chars().nth(2).map(parse_answer).flatten(),
        Strategy::Outcome => s.chars().nth(2).map(|c| get_outcome(c, opponent)).flatten(),
    };

    match (opponent, player) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

fn points(game: (Move, Move)) -> i32 {
    let (opponent, player) = game;

    let shape_score = match player {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let win_score = match player.partial_cmp(&opponent) {
        Some(std::cmp::Ordering::Greater) => 6,
        Some(std::cmp::Ordering::Equal) => 3,
        _ => 0,
    };
    shape_score + win_score
}

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day02.txt")?;
    let reader = BufReader::new(file);

    let strategy = match args.get(0).map(|s| s.parse::<i32>()) {
        Some(Ok(1)) => Strategy::Answer,
        _ => Strategy::Outcome,
    };

    let score = reader
        .lines()
        .filter_map(|x| match x {
            Ok(x) => Some(x),
            _ => None,
        })
        .map(|s| parse(s, strategy))
        .filter_map(|x| x)
        .fold(0, |p, g| p + points(g));

    println!("Scored: {}", score);
    Ok(())
}
