use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(args: &[String]) -> std::io::Result<()> {
    println!("day 01");
    let file = File::open("input/day01.txt")?;
    let reader = BufReader::new(file);

    let mut heap = BinaryHeap::new();
    let n = args.get(0).map(|s| s.parse::<i32>());

    let x = match n {
        Some(Ok(v)) => v,
        _ => 1,
    };

    for _ in 0..x {
        heap.push(Reverse(0));
    }

    reader
        .lines()
        .filter_map(|l| match l {
            Ok(s) => Some(s),
            _ => None,
        })
        .map(|s| s.parse::<i32>())
        .fold(0, |elf, n| match n {
            Ok(v) => elf + v,
            _ => match heap.peek() {
                Some(Reverse(min)) => {
                    if *min < elf {
                        heap.pop();
                        heap.push(Reverse(elf));
                        println!("{}", elf);
                        0
                    } else {
                        println!("{}", elf);
                        0
                    }
                }
                _ => elf,
            },
        });

    let calories = heap.iter().fold(0, |a, b| a + b.0);
    println!("{} Elves with most calories: {}", x, calories);
    Ok(())
}
