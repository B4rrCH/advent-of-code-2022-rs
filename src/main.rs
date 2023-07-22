mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

use crate::day01::run as run01;
use crate::day02::run as run02;
use crate::day03::run as run03;
use crate::day04::run as run04;
use crate::day05::run as run05;
use crate::day06::run as run06;
use crate::day07::run as run07;
use crate::day08::run as run08;
use crate::day09::run as run09;
use crate::day10::run as run10;

use std::env;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let day = args.get(1).map(|s| s.parse::<i32>().unwrap_or(1));

    let args_for_day = &args[2..];
    match day {
        Some(1) => run01(args_for_day),
        Some(2) => run02(args_for_day),
        Some(3) => run03(args_for_day),
        Some(4) => run04(args_for_day),
        Some(5) => run05(args_for_day),
        Some(6) => run06(args_for_day),
        Some(7) => run07(args_for_day),
        Some(8) => run08(args_for_day),
        Some(9) => run09(args_for_day),
        Some(10) => run10(args_for_day),
        _ => {
            panic!("Day {:?} not implemented", day);
        }
    }
}
