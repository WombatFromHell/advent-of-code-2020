use std::{env, fs, path, io};
use fs::File;
use io::{BufReader, Error};
use path::{Path, PathBuf};
use std::io::prelude::*;

fn read_to_vec(path: PathBuf) -> Result<Vec<i32>, Error> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let lines = buf.lines()
        .map(|l| l.expect("Could not parse line!"))
        .map(|l| l.parse::<i32>().unwrap())
        .collect();
    Ok(lines)
}

fn part_a(_vec: Vec<i32>) -> Option<i32> {
    for (_i, _x) in _vec.iter().enumerate() {
        for (_j, _y) in _vec.iter().enumerate() {
            if _x + _y == 2020 {
                return Some(_x * _y);
            }
        }
    }
    return None;
}

fn part_b(_vec: Vec<i32>) -> Option<i32> {
    for (_i, _x) in _vec.iter().enumerate() {
        for (_j, _y) in _vec.iter().enumerate() {
            for (_k, _z) in _vec.iter().enumerate() {
                if _x + _y + _z == 2020 {
                    return Some(_x * _y * _z);
                }
            }
        }
    }
    return None;
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let arg_path = args.as_slice().get(1).unwrap();
    let filepath = Path::new(&arg_path).to_path_buf();

    let _vec = read_to_vec(filepath)?;
    let _vec2 = _vec.clone();

    if let Some(a) = part_a(_vec) {
        println!("solved for A: {}", a);
    } else {
        eprintln!("found no solution for part A!");
    };

    if let Some(b) = part_b(_vec2) {
        println!("solved for B: {}", b);
    } else {
        eprintln!("found no solution for part B!");
    };

    Ok(())
}
