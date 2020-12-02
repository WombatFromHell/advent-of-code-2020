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

async fn part_a(arg_path: &String) -> Result<Vec<i32>, Error> {
    let filepath = Path::new(&arg_path).to_path_buf();
    let _a = read_to_vec(filepath).await?;
    let _b = &_a.clone();
    let mut result = 0;
    // part A
    for (_i, _x) in _a.iter().enumerate() {
        for (_j, _y) in _b.iter().enumerate() {
            if _x + _y == 2020 {
                result = _x * _y;
                break;
            }
        }
        if result > 0 {
            break;
        }
    }
    println!("part A: {}", result);
    Ok(_a)
}

fn part_b(report: Vec<i32>) {
    let mut result = 0;
    // part B
    for (_i, _x) in report.iter().enumerate() {
        if result > 0 {
            break;
        }
        for (_j, _y) in report.iter().enumerate() {
            if result > 0 {
                break;
            }
            for (_k, _z) in report.iter().enumerate() {
                if _x + _y + _z == 2020 {
                    result = _x * _y * _z;
                    break;
                }
            }
        }
    }
    println!("part B: {}", result);
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let firstmatch = args.as_slice().get(1).unwrap();
    let result_a = part_a(&firstmatch).await?;
    part_b(result_a);
    Ok(())
}
