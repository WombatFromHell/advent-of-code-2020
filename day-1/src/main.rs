use std::env;
use async_std::{io::Error, path::PathBuf, io::BufReader};
use async_std::{fs::File, path::Path};
use async_std::prelude::*;

async fn read_to_vec(path: PathBuf) -> Result<Vec<i32>, Error> {
    let file = File::open(&path).await?;
    let filebuf = BufReader::new(file);
    let mut lines = filebuf.lines();
    let mut contents: Vec<i32> = Vec::new();
    while let Some(line) = lines.next().await {
        let val = line?;
        let parsed = val.parse::<i32>();
        contents.push(parsed.unwrap());
    }
    Ok(contents)
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

#[async_std::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let arg_path = args.as_slice().get(1).unwrap();
    let filepath = Path::new(&arg_path).to_path_buf();

    let _vec = read_to_vec(filepath).await?;
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
