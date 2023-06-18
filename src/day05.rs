use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;
use std::iter::*;

fn parse_crate_image(lines_of_image: &Vec<String>) -> Vec<Vec<char>> {
    let height = lines_of_image.len() - 1;
    let width = ((lines_of_image[0].len()) + 1) / 4;

    let mut res = Vec::new();

    for _ in 0..width {
        res.push(Vec::new());
    }

    for i in 0..width {
        let x = 4 * i + 1;
        for j in 0..height {
            let stack = res.get_mut(i).unwrap();
            let y = height - j - 1;

            if let Some(Some(c)) = lines_of_image
                .get(y)
                .map(|line| line.chars().skip(x).next())
            {
                if c == ' ' {
                    break;
                }
                stack.push(c);
            }
        }
    }
    res
}

fn get_image(lines: &mut Lines<BufReader<File>>) -> Vec<String> {
    return lines
        .filter_map(Result::ok)
        .take_while(|x| *x != "")
        .collect();
}

fn get_move(s: String) -> Option<(usize, usize, usize)> {
    let words: Vec<_> = s.split(' ').collect();

    let how_many = words
        .get(1)
        .map(|s| s.parse::<usize>())
        .map(Result::ok)
        .flatten();
    let from = words
        .get(3)
        .map(|s| s.parse::<usize>())
        .map(Result::ok)
        .flatten();
    let to = words
        .get(5)
        .map(|s| s.parse::<usize>())
        .map(Result::ok)
        .flatten();

    match (how_many, from, to) {
        (Some(how_many), Some(from), Some(to)) => Some((how_many, from - 1, to - 1)),
        _ => None,
    }
}

fn run_part1(reader: BufReader<File>) -> std::io::Result<()> {
    let mut lines = reader.lines();
    let image = get_image(&mut lines);
    let mut stacks = parse_crate_image(&image);

    for movement in lines.filter_map(Result::ok).filter_map(get_move) {
        let (how_many, from, to) = movement;
        for _ in 0..how_many {
            if let Some(c) = stacks[from].pop() {
                stacks[to].push(c);
            }
        }
    }

    let res = stacks
        .iter()
        .map(|stack| stack.last())
        .filter_map(|x| x)
        .collect::<String>();
    println!("The top crates are {}", res);

    Ok(())
}

fn run_part2(reader: BufReader<File>) -> std::io::Result<()> {
    let mut lines = reader.lines();
    let image = get_image(&mut lines);
    let mut stacks = parse_crate_image(&image);

    for movement in lines.filter_map(Result::ok).filter_map(get_move) {
        let (how_many, from, to) = movement;
        let mut tmp = Vec::new();
        for _ in 0..how_many {
            if let Some(c) = stacks[from].pop() {
                tmp.push(c);
            }
        }

        while let Some(c) = tmp.pop() {
            stacks[to].push(c);
        }
    }

    let res = stacks
        .iter()
        .map(|stack| stack.last())
        .filter_map(|x| x)
        .collect::<String>();
    println!("The top crates are {}", res);

    Ok(())
}

pub fn run(args: &[String]) -> std::io::Result<()> {
    let file = File::open("input/day05.txt")?;
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
