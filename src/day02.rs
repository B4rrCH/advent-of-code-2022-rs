use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day02.txt")?;
    let reader = BufReader::new(file);

    let strategy = args
        .get(0)
        .and_then(|s| s.try_into().ok())
        .unwrap_or(Strategy::Answer);

    let score: usize = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|s| {
            let mut chars = s.chars();
            let opponent: Option<Move> = chars.next().and_then(|c| c.try_into().ok());

            // Skip the space
            _ = chars.next();

            let player: Option<Move> = chars.next().and_then(|c| match strategy {
                Strategy::Answer => c.try_into().ok(),
                Strategy::Outcome => match c {
                    // player loses
                    'X' => match opponent {
                        Some(Move::Scissors) => Some(Move::Paper),
                        Some(Move::Paper) => Some(Move::Rock),
                        Some(Move::Rock) => Some(Move::Scissors),
                        _ => None,
                    },

                    // player draws
                    'Y' => opponent,

                    // player wins
                    'Z' => match opponent {
                        Some(Move::Paper) => Some(Move::Scissors),
                        Some(Move::Rock) => Some(Move::Paper),
                        Some(Move::Scissors) => Some(Move::Rock),
                        _ => None,
                    },
                    _ => None,
                },
            });

            match (opponent, player) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            }
        })
        .map(get_player_score_for_game)
        .sum();

    println!("Scored: {}", score);
    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Strategy {
    Answer,
    Outcome,
}

impl TryFrom<&String> for Strategy {
    type Error = ();
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "1" => Ok(Self::Answer),
            "2" => Ok(Self::Outcome),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

fn get_player_score_for_game(game: (Move, Move)) -> usize {
    let (opponent, player) = game;
    let shape_score = match player {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    let win_score = match (player, opponent) {
        (x, y) if x == y => 3,
        (Move::Rock, Move::Scissors)
        | (Move::Scissors, Move::Paper)
        | (Move::Paper, Move::Rock) => 6,
        _ => 0,
    };
    shape_score + win_score
}
