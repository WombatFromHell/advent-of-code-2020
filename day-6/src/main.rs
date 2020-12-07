use path::{Path, PathBuf};
use std::collections::HashSet;
use std::error::Error;
use std::io::prelude::*;
use std::{env, fs, path};

fn read_to_chunks(path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = fs::File::open(path)?;
    let mut _buf = String::new();
    file.read_to_string(&mut _buf)?;
    // strip windows' carriage returns
    let lines = _buf
        .replace("\r", "")
        .split("\n\n")
        .enumerate()
        .map(|(_, s)| s.to_owned())
        .collect::<Vec<String>>();
    Ok(lines)
}

fn part_a(input: &Vec<String>) -> usize {
    let result: usize = input
        .iter()
        .map(|g| {
            g.chars()
                .filter(|&c| !c.is_whitespace())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!("part A: questions answered 'yes' {}", result);
    result
}

fn part_b(input: &Vec<String>) -> usize {
    let result: usize = input
        .iter()
        .map(|g| {
            g.lines()
                .map(|c| c.chars().filter(|&c| c != '\n').collect::<HashSet<_>>())
                .fold(('a'..='z').collect::<HashSet<_>>(), |acc, c| {
                    // use HashSet intersection to find uniques
                    acc.intersection(&c).cloned().collect()
                })
                .len()
        })
        .sum();
    println!("part B: questions everyone answered 'yes' to {}", result);
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg_path) = args.as_slice().get(1) {
        let filepath = Path::new(&arg_path).to_path_buf();
        let as_groups = read_to_chunks(&filepath)?;
        part_a(&as_groups);
        part_b(&as_groups);
    }
    Ok(())
}
