mod day01;
mod day02;
mod day03;

use crate::day01::run as run01;
use crate::day02::run as run02;
use crate::day03::run as run03;

use std::{collections::VecDeque, env};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut args: VecDeque<String> = env::args().collect();
    _ = args.pop_front();

    let day = args.pop_front().map(|s| match s.parse::<i32>() {
        Ok(i) => i,
        Err(_) => 1,
    });
    let args: Vec<String> = args.iter().map(|s| s.to_owned()).collect();
    dbg!(day);
    match day {
        Some(1) => run01(&args)?,
        Some(2) => run02(&args)?,
        Some(3) => run03(&args)?,
        _ => println!("Please provide a day"),
    };
    Ok(())
}
