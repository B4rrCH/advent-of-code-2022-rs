use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(args: &[String]) -> std::io::Result<()> {
    let tree_heights = read_tree_heights_from_file("input/day08.txt")?;

    match args.get(0).and_then(|s| s.parse::<i32>().ok()) {
        Some(1i32) => part1(tree_heights),
        Some(2i32) => part2(tree_heights),
        _ => panic!("Unknown part"),
    }

    Ok(())
}

fn part1(tree_heights: Vec<Vec<u8>>) {
    let mut tree_hights_and_visibility: Vec<Vec<_>> = tree_heights
        .iter()
        .map(|line| line.iter().copied().map(|height| (height, false)).collect())
        .collect();

    let height = tree_hights_and_visibility.len();
    let width = tree_hights_and_visibility[0].len();

    for line in &mut tree_hights_and_visibility {
        // Left to right
        let mut highest: Option<u8> = None;
        for (height, is_visible) in line.iter_mut() {
            if highest.is_none() || highest.unwrap() < *height {
                highest = Some(*height);
                *is_visible = true;
            }
        }

        // Right to left
        highest = None;
        for j in (0..width).rev() {
            let (height, is_visible) = &mut line[j];
            if highest.is_none() || highest.unwrap() < *height {
                highest = Some(*height);
                *is_visible = true;
            }
        }
    }

    for j in 0..width {
        // Top to bottom
        let mut highest: Option<u8> = None;
        for (height, is_visible) in &mut tree_hights_and_visibility[j] {
            if highest.is_none() || highest.unwrap() < *height {
                highest = Some(*height);
                *is_visible = true;
            }
        }

        // Bottom to top
        let mut highest: Option<u8> = None;
        for i in (0..height).rev() {
            let (height, is_visible) = &mut tree_hights_and_visibility[i][j];
            if highest.is_none() || highest.unwrap() < *height {
                highest = Some(*height);
                *is_visible = true;
            }
        }
    }

    let nr_of_visible_trees =
        tree_hights_and_visibility
            .iter()
            .fold(0usize, |nr_of_visible_trees, line| {
                nr_of_visible_trees
                    + line
                        .iter()
                        .map(|(_, is_visible)| *is_visible as usize)
                        .sum::<usize>()
            });

    println!("There are {} trees visible", nr_of_visible_trees);
}

fn part2(tree_heights: Vec<Vec<u8>>) {
    let maximum_scenic_score = tree_heights
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, _)| scenic_score(i, j, &tree_heights))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("The maximum scenic score is {}", maximum_scenic_score);
}

fn scenic_score(i: usize, j: usize, tree_heights: &Vec<Vec<u8>>) -> usize {
    let height = tree_heights.len();
    let width = tree_heights[0].len();

    let mut up: usize = 0;
    for ii in (0..i).rev() {
        up += 1;
        if tree_heights[ii][j] >= tree_heights[i][j] {
            break;
        }
    }

    let mut down: usize = 0;
    for ii in (i + 1)..height {
        down += 1;
        if tree_heights[ii][j] >= tree_heights[i][j] {
            break;
        }
    }

    let mut left: usize = 0;
    for jj in (0..j).rev() {
        left += 1;
        if tree_heights[i][jj] >= tree_heights[i][j] {
            break;
        }
    }

    let mut right: usize = 0;
    for jj in (j + 1)..width {
        right += 1;
        if tree_heights[i][jj] >= tree_heights[i][j] {
            break;
        }
    }

    up * down * left * right
}

fn read_tree_heights_from_file(path: &str) -> std::io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut tree_hights_and_visibility: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        tree_hights_and_visibility.push(line?.as_bytes().to_vec());
    }
    Ok(tree_hights_and_visibility)
}
