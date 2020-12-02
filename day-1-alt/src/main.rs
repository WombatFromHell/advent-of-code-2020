use std::env;
use async_std::{io::Error, path::PathBuf, io::BufReader};
use async_std::{fs::File, path::Path};
use async_std::prelude::*;

#[derive(Debug)]
struct SumTwo {
    x: i32,
    y: i32,
    sum: i32,
    product: i32,
}
#[derive(Debug)]
struct SumThree {
    x: i32,
    y: i32,
    z: i32,
    sum: i32,
    product: i32,
}

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

fn sum_two(_vec: Vec<i32>) -> Option<SumTwo> {
    let length = _vec.len();
    let mut attempts = 0;
    for i in 0..length-2 {
        for j in 0..i+1 {
            let x = _vec[i];
            let y = _vec[j];
            let sum = x + y;
            attempts += 1;
            if sum == 2020 {
                let product = x * y;
                let result = SumTwo { x, y, sum, product };
                println!("{:?} after {} attempts", result, attempts);
                return Some(result);
            }
        }
    }
    println!("found no solution for two-sum!");
    None
}

fn sum_three(_vec: Vec<i32>) -> Option<SumThree> {
    //let hashmap = build_hashmap(_vec);
    let length = _vec.len();
    let mut attempts = 0;

    for i in 0..length-2 {
        for j in 0..i+1 {
            for k in 0..length {
                let x = _vec[i];
                let y = _vec[j];
                let z = _vec[k];
                let sum = x + y + z;
                attempts += 1;
                if sum == 2020 {
                    let product = x * y * z;
                    let result = SumThree { x, y, z, sum, product };
                    println!("{:?} after {} attempts", result, attempts);
                    return Some(result);
                }
            }
        }
    }
    println!("found no solution for three-sum!");
    None
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let arg_path = args.as_slice().get(1).unwrap();
    let filepath = Path::new(&arg_path).to_path_buf();

    let mut _vec = read_to_vec(filepath).await?;
    _vec.sort();
    let _vec2 = _vec.clone();

    sum_two(_vec);
    sum_three(_vec2);

    Ok(())
}
