use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_range(s: &str) -> Option<(i32, i32)> {
    let numbers: Vec<_> = s.split('-').map(str::parse::<i32>).collect();
    if numbers.len() == 2 {
        if let (Some(Ok(start)), Some(Ok(end))) = (numbers.get(0), numbers.get(1)) {
            return Some((*start, *end));
        }
    }
    None
}

fn parse_line(line: String) -> Option<((i32, i32), (i32, i32))> {
    let ranges: Vec<_> = line.split(',').map(parse_range).collect();
    if ranges.len() == 2 {
        if let (Some((a, b)), Some((c, d))) = (ranges[0], ranges[1]) {
            return Some(((a, b), (c, d)));
        }
    }
    None
}

fn either_fully_contained(ranges: ((i32, i32), (i32, i32))) -> bool {
    let ((a, b), (c, d)) = ranges;
    (a <= c && c <= d && d <= b) || (c <= a && a <= b && b <= d)
}

fn overlap(ranges: ((i32, i32), (i32, i32))) -> bool {
    let ((a, b), (c, d)) = ranges;
    (a <= c && c <= b) || (a <= d && d <= b) || (c <= a && a <= d) || (c <= b && b <= d)
}

fn run_part1(reader: BufReader<File>) -> std::io::Result<()> {
    let number_fully_contained = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(parse_line)
        .map(either_fully_contained)
        .fold(0, |a, b| a + if b { 1 } else { 0 });

    println!(
        "The number of fully contained ranges is {}",
        number_fully_contained
    );

    Ok(())
}

fn run_part2(reader: BufReader<File>) -> std::io::Result<()> {
    let number_fully_contained = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(parse_line)
        .map(overlap)
        .fold(0, |a, b| a + if b { 1 } else { 0 });

    println!(
        "The number of overlapping ranges is {}",
        number_fully_contained
    );

    Ok(())
}

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day04.txt")?;
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
