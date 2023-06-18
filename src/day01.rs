use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(args: &[String]) -> std::io::Result<()> {
    println!("day 01");
    let file = File::open("input/day01.txt")?;
    let reader = BufReader::new(file);

    let number_of_top_elves = args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(1);

    let mut top_elves_calories = BinaryHeap::new();
    for _ in 0..number_of_top_elves {
        top_elves_calories.push(Reverse(0));
    }

    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<i32>().ok())
        .fold(
            0,
            |calories_of_current_elf, maybe_calories| match maybe_calories {
                Some(calories) => calories_of_current_elf + calories,
                _ => match top_elves_calories.peek() {
                    Some(Reverse(min)) => {
                        if *min < calories_of_current_elf {
                            top_elves_calories.pop();
                            top_elves_calories.push(Reverse(calories_of_current_elf));
                            0
                        } else {
                            0
                        }
                    }
                    _ => calories_of_current_elf,
                },
            },
        );

    let calories = top_elves_calories.iter().fold(0, |a, b| a + b.0);
    println!(
        "The {} elves with the most calories carry {}",
        number_of_top_elves, calories
    );
    Ok(())
}
