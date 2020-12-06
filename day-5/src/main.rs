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

fn find_highest(chunks: Vec<String>) -> Option<Vec<i32>> {
    let mut results: Vec<i32> = Vec::new();
    for line in chunks {
        // map each boarding pass to a seat_id in binary (10 bits -> 1024)
        let _mapped = line[0..10]
            .chars()
            .enumerate()
            .map(|(_, c)| match c {
                'B' | 'R' => '1',
                _ => '0',
            })
            .collect::<String>();
        let seat_id = i32::from_str_radix(&_mapped[..], 2).unwrap();
        if seat_id < 1024 {
            results.push(seat_id);
        }
    }
    if results.len() > 0 {
        return Some(results);
    }
    None
}

fn find_seat(seats: &Vec<i32>, highest: &i32) -> Option<i32> {
    let mut seats = seats.clone();
    seats.sort_by(|a, b| a.cmp(b));
    // find the first window that isn't sequential
    for window in seats.windows(2) {
        let seatid = window[0]+1;
        if seatid != window[1] && seatid <= *highest {
            return Some(window[0]+1);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg_path) = args.as_slice().get(1) {
        let filepath = Path::new(&arg_path).to_path_buf();
        let lines = read_in_lines(filepath)?;
        if let Some(result_a) = find_highest(lines) {
            let highest_id = result_a.iter().max().unwrap();
            println!("part A: highest seat id = {}", &highest_id);

            if let Some(result_b) = find_seat(&result_a, &highest_id) {
                println!("part B: your seat id is = {}", result_b);
            } else {
                println!("part B: could not find your seat id!");
            }
        } else {
            println!("part A: could not find the highest seat id!");
        }
    }
    Ok(())
}
