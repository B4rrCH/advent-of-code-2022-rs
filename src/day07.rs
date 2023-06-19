use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;

enum Output {
    ChangeDirectory(String),
    List,
    File(usize, String),
    Directory(String),
}

struct File {
    size: usize,
    file_name: String,
}

#[derive(Debug)]
struct Directory {
    sub_directories: HashMap<String, Directory>,
    files: HashMap<String, usize>,
}

impl Directory {
    fn get_inclusive_size(&self) -> usize {
        self.sub_directories
            .values()
            .map(|sub_directory| sub_directory.get_inclusive_size())
            .sum::<usize>()
            + self.files.values().sum::<usize>()
    }

    fn get_sub_directories(&self, path: &str) -> HashMap<String, &Directory> {
        let mut res = HashMap::new();
        res.insert(path.into(), self);

        for (name, sub_dir) in self.sub_directories.iter() {
            let sub_path = format!("{}/{}", path, name.as_str());
            let sub_dirs = sub_dir.get_sub_directories(sub_path.as_str());
            res.extend(sub_dirs);
        }

        res
    }
}

impl TryFrom<String> for Output {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("$ cd ") {
            Ok(Output::ChangeDirectory(
                value.chars().skip("$ cd ".len()).collect(),
            ))
        } else if value.starts_with("$ ls") {
            Ok(Output::List)
        } else if value.starts_with("dir ") {
            Ok(Output::Directory(
                value.chars().skip("dir ".len()).collect(),
            ))
        } else if let Some((Some(size), file_name)) = value
            .split_once(' ')
            .map(|(s, file_name)| (s.parse::<usize>().ok(), String::from(file_name)))
        {
            Ok(Output::File(size, file_name))
        } else {
            Err(())
        }
    }
}

pub fn run(_args: &[String]) -> std::io::Result<()> {
    let file = std::fs::File::open("input/day07.txt")?;
    let reader = BufReader::new(file);

    let root = get_directory_tree(&mut reader.lines());

    let all_directories = root.get_sub_directories("");

    let sizes_of_directories: Vec<usize> = all_directories
        .values()
        .map(|dir| dir.get_inclusive_size())
        .collect();

    let max_size_for_part_1 = 100_000;
    let sum_of_inclusive_sizes: usize = sizes_of_directories
        .iter()
        .filter(|size| **size < max_size_for_part_1)
        .sum();
    println!(
        "The sum of the inclusive sizes less than {} is {}",
        max_size_for_part_1, sum_of_inclusive_sizes
    );

    let total_size: usize = 70000000;
    let needed_free_space: usize = 30000000;
    let total_used_size = root.get_inclusive_size();
    let needed_to_delete = total_used_size - (total_size - needed_free_space);

    let min_feasible_size = sizes_of_directories
        .iter()
        .filter(|size| **size >= needed_to_delete)
        .min()
        .unwrap();

    println!(
        "The smallest directory size over {} is {}",
        needed_to_delete, min_feasible_size
    );

    Ok(())
}

fn get_directory_tree(lines: &mut Lines<BufReader<std::fs::File>>) -> Directory {
    let mut current_path: Vec<String> = vec![];
    let mut root = Directory {
        files: HashMap::new(),
        sub_directories: HashMap::new(),
    };

    while let Some(output) = lines
        .next()
        .and_then(Result::ok)
        .map(Output::try_from)
        .and_then(Result::ok)
    {
        match output {
            Output::ChangeDirectory(relative_path) => match relative_path.as_str() {
                "/" => current_path.clear(),
                ".." => _ = current_path.pop(),
                _ => {
                    current_path.push(relative_path);
                    ensure_directory_exists(&mut root, &current_path);
                }
            },
            Output::List => (),
            Output::File(size, file_name) => {
                add_file(&mut root, &current_path, File { size, file_name });
            }
            Output::Directory(name) => {
                current_path.push(name);
                ensure_directory_exists(&mut root, &current_path);
                current_path.pop();
            }
        };
    }
    root
}

fn add_file(root: &mut Directory, current_path: &[String], file: File) {
    match current_path.len() {
        0 => {
            root.files.insert(file.file_name, file.size);
        }
        _ => {
            let (sub_directory_name, current_path) = (&current_path[0], &current_path[1..]);
            add_file(
                root.sub_directories.get_mut(sub_directory_name).unwrap(),
                current_path,
                file,
            )
        }
    }
}

fn ensure_directory_exists(root: &mut Directory, current_path: &[String]) {
    let (sub_directory_name, current_path) = (current_path[0].clone(), &current_path[1..]);

    if !root.sub_directories.contains_key(&sub_directory_name) {
        root.sub_directories.insert(
            sub_directory_name.clone(),
            Directory {
                sub_directories: HashMap::new(),
                files: HashMap::new(),
            },
        );
    }

    if !current_path.is_empty() {
        ensure_directory_exists(
            root.sub_directories.get_mut(&sub_directory_name).unwrap(),
            current_path,
        );
    }
}
