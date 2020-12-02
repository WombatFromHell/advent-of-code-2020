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

fn sum_two(_vec: Vec<i32>) -> Option<i32> {
    let length = _vec.len();
    let mut attempts = 0;
    for i in 0..length-2 {
        for j in 0..i+1 {
            let x = _vec[i];
            let y = _vec[j];
            attempts += 1;
            if x + y == 2020 {
                let product = x * y;
                println!("found {} after {} attempts", product, attempts);
                return Some(product);
            }
        }
    }
    println!("found no solution for two-sum!");
    None
}

fn sum_three(_vec: Vec<i32>) -> Option<i32> {
    let length = _vec.len();
    let mut attempts = 0;

    for i in 0..length-2 {
        for j in 0..i+1 {
            for k in 0..length {
                let x = _vec[i];
                let y = _vec[j];
                let z = _vec[k];
                attempts += 1;
                if x + y + z == 2020 {
                    let product = x * y * z;
                    println!("found {} after {} attempts", product, attempts);
                    return Some(product);
                }
            }
        }
    }
    println!("found no solution for three-sum!");
    None
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg_path) = args.as_slice().get(1) {
        let filepath = Path::new(&arg_path).to_path_buf();

        let mut _vec = read_to_vec(filepath)?;
        _vec.sort();
        let _vec2 = _vec.clone();

        sum_two(_vec);
        sum_three(_vec2);
    };

    Ok(())
}
