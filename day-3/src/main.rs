use std::{env, fs, path, io};
use fs::File;
use io::{BufReader, Error};
use path::{Path, PathBuf};
use std::io::prelude::*;

fn read_to_vec(path: PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let lines = buf.lines()
        .map(|l| l.expect("Could not parse line!"))
        .collect();
    Ok(lines)
}

fn part_a(_vec: Vec<String>, slope: (usize, usize)) -> i32 {
    let mut trees = 0;
    let mut x = slope.0;
    let mut y = slope.1;
    while y < _vec.len() {
        let mut _extstr = _vec[y].to_owned();
        while x >= _extstr.len() {
            // extend the line if it's too short
            _extstr.push_str(&_vec[y]);
        }
        let _chars: Vec<char> = _extstr.chars().collect();
        if _chars[x] == '#' {
            trees += 1;
        }
        x += slope.0;
        y += slope.1;
    }
    trees
}

fn part_b(_vec: Vec<String>) -> i64 {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product: i64 = 0;
    for i in 0..slopes.len() {
        let result = part_a(_vec.clone(), slopes[i]);
        if i == 0 {
            product = result as i64;
        } else {
            product *= result as i64;
        }
    }
    product
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg_path) = args.as_slice().get(1) {
        let filepath = Path::new(&arg_path).to_path_buf();

        let _vec = read_to_vec(filepath)?;
        let _vec2 = _vec.clone();

        let result_a = part_a(_vec, (3, 1));
        if result_a > 0 {
            println!("part A: hit {} trees", result_a);
        } else {
            println!("part A: hit no trees!");
        }

        let result_b = part_b(_vec2);
        if result_b > 0 {
            println!("part B: product of slopes: {}", result_b);
        } else {
            println!("part B: returned no product!");
        }
    }

    Ok(())
}