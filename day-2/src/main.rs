use text_io::scan;
use std::env;
use async_std::{io::Error, path::PathBuf, io::BufReader};
use async_std::{fs::File, path::Path};
use async_std::prelude::*;

#[derive(Clone, Debug)]
struct ParsedPasswd {
    min: usize,
    max: usize,
    letter: String,
    passwd: String,
    occurrences: usize,
}

async fn read_to_vec(path: PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(&path).await?;
    let filebuf = BufReader::new(file);
    let mut lines = filebuf.lines();
    let mut contents: Vec<String> = Vec::new();
    while let Some(line) = lines.next().await {
        contents.push(line.unwrap());
    }
    Ok(contents)
}

fn parse_line(input: String) -> ParsedPasswd {
    let mut min = 0;
    let mut max = 0;
    let mut letter = String::new();
    let mut passwd = String::new();
    scan!(input.bytes() => "{}-{} {}: {}", min, max, letter, passwd);
    let occurrences = input.matches(&letter.clone()).count()-1;
    ParsedPasswd {
        min,
        max,
        letter,
        passwd,
        occurrences
    }
}

#[allow(unused_assignments)]
fn part_a (_vec: Vec<String>) -> i32 {
    let mut valid = 0;
    for l in _vec {
        let parsed = parse_line(l);
        let occ = parsed.occurrences;
        let min = parsed.min;
        let max = parsed.max;
        if occ >= min && occ <= max {
            valid += 1;
        }
    }
    valid
}

fn part_b (_vec: Vec<String>) -> i32 {
    let mut validated = 0;
    for l in _vec {
        let parsed = parse_line(l);
        let _parsed = parsed.clone();
        let ltr = parsed.letter;
        let idx0 = parsed.min-1;
        let idx1 = parsed.max-1;
        let pwd = parsed.passwd;
        let _asb = pwd.as_bytes();
        let _first_char = std::str::from_utf8(&_asb[idx0..idx0+1]).unwrap();
        let _sec_char = std::str::from_utf8(&_asb[idx1..idx1+1]).unwrap();

        if _first_char == &ltr && _sec_char != &ltr || _first_char != &ltr && _sec_char == &ltr{
            validated += 1;
        }
    }
    validated
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let arg_path = args.as_slice().get(1).unwrap();
    let filepath = Path::new(&arg_path).to_path_buf();

    let _vec = read_to_vec(filepath).await?;
    let _vec2 = _vec.clone();
    let _vec_len = _vec.len();
    let results_a = part_a(_vec);
    if results_a > 0 {
        println!("part A found {} valid passwords out of {}", results_a, _vec_len);
    } else {
        println!("part A found no valid passwords!");
    }
    let results_b = part_b(_vec2);
    if results_b > 0 {
        println!("part B found {} valid passwords out of {}", results_b, _vec_len);
    } else {
        println!("part B found no valid passwords!");
    }

    Ok(())
}
