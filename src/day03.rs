use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_item_in_both(str: String) -> Option<char> {
    let l = str.len();

    let mut first_half = HashSet::new();

    for c in str.chars().take(l / 2) {
        first_half.insert(c);
    }
    str.chars().skip(l / 2).find(|&c| first_half.contains(&c))
}

fn get_priority(c: char) -> Option<i32> {
    match c {
        'a'..='z' => Some(c as i32 - 'a' as i32 + 1),
        'A'..='Z' => Some(c as i32 - 'A' as i32 + 27),
        _ => None,
    }
}

fn run_part1(reader: BufReader<File>) -> std::io::Result<()> {
    let sum_of_priorities: i32 = reader
        .lines()
        .filter_map(|x| match x {
            Ok(x) => Some(x),
            _ => None,
        })
        .filter_map(get_item_in_both)
        .filter_map(get_priority)
        .sum();

    println!("The sum of priorities is {}", sum_of_priorities);
    Ok(())
}

fn run_part2(reader: BufReader<File>) -> std::io::Result<()> {
    let sum: i32 = reader
        .lines()
        .map_while(Result::ok)
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|x| x.chars().collect::<HashSet<char>>())
                .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<char>>())
                .and_then(|s| s.into_iter().collect::<Vec<char>>().first().copied())
                .and_then(get_priority)
                .unwrap_or(0)
        })
        .sum();

    println!("Sum of priorities {:?}", sum);
    Ok(())
}

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day03.txt")?;
    let reader = BufReader::new(file);

    match args.get(0).map(|s| s.parse::<i32>()) {
        Some(Ok(1)) => run_part1(reader),
        Some(Ok(2)) => run_part2(reader),
        _ => {
            println!("Unknown part");
            Ok(())
        }
    }?;

    Ok(())
}
