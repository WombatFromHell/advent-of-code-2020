use path::{Path, PathBuf};
use std::error::Error;
use std::io::prelude::*;
use std::{env, fs, path};

fn read_in_lines(path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut _buf = String::new();
    let mut file = fs::File::open(&path)?;
    file.read_to_string(&mut _buf)?;
    Ok(_buf.lines().map(|l| l.to_owned()).collect::<Vec<String>>())
}

fn bp_solver(chunks: Vec<String>) -> Option<Vec<i32>> {
    let mut results: Vec<i32> = Vec::new();
    for line in chunks {
        // map each boarding pass to a seat_id in binary (10 bits -> 1023)
        let _mapped = line[0..10]
            .chars()
            .enumerate()
            .map(|(_, c)| match c {
                'B' | 'R' => '1',
                _ => '0',
            })
            .collect::<String>();
        let seat_id = i32::from_str_radix(&_mapped[..], 2).unwrap();
        if seat_id <= 1023 {
            results.push(seat_id);
        }
    }
    if results.len() > 0 {
        return Some(results);
    }
    None
}

fn part_b(seats: &Vec<i32>, highest: &i32) -> Option<i32> {
    let seats = seats.clone();
    for i in 0..highest.to_owned() {
        let has_low = seats.contains(&(i-1));
        let has_high = seats.contains(&(i+1));
        if has_low && has_high && !seats.contains(&i) {
            return Some(i);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg_path) = args.as_slice().get(1) {
        let filepath = Path::new(&arg_path).to_path_buf();

        if let Ok(lines) = read_in_lines(filepath) {
            if let Some(result_a) = bp_solver(lines) {
                let highest = result_a.iter().max().unwrap();
                println!("part A: highest calculated seat id {}", highest);

                if let Some(result_b) = part_b(&result_a, &highest) {
                    println!("part B: your seat is {}", result_b);
                } else {
                    println!("part B: could not find your seat!");
                }
            }
        }
    }
    Ok(())
}
