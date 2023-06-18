mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use crate::day01::run as run01;
use crate::day02::run as run02;
use crate::day03::run as run03;
use crate::day04::run as run04;
use crate::day05::run as run05;

use std::env;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let day = args.get(1).map(|s| match s.parse::<i32>() {
        Ok(i) => i,
        Err(_) => 1,
    });

    let args_for_day = &args[2..];
    dbg!(day);
    match day {
        Some(1) => run01(&args_for_day),
        Some(2) => run02(&args_for_day),
        Some(3) => run03(&args_for_day),
        Some(4) => run04(&args_for_day),
        Some(5) => run05(&args_for_day),
        _ => {
            panic!("Day {:?} not implemented", day);
        }
    }
}
