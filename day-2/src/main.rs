use text_io::scan;
use std::{str, env, fs, path, io};
use fs::File;
use io::{BufReader, Error};
use path::{Path, PathBuf};
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct ParsedPasswd {
    min: usize,
    max: usize,
    letter: String,
    passwd: String,
    occurrences: usize,
}

fn read_to_vec(path: PathBuf) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let lines = buf.lines()
        .map(|l| l.expect("Could not parse line!"))
        .collect();
    Ok(lines)
}

#[allow(unused_assignments)]
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
        if let Ok(_first_char) = str::from_utf8(&_asb[idx0..idx0+1]) {
            if let Ok(_sec_char) = str::from_utf8(&_asb[idx1..idx1+1]) {
                if _first_char == &ltr && _sec_char != &ltr || _first_char != &ltr && _sec_char == &ltr {
                    validated += 1;
                }
            };
        };
    }
    validated
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg_path) = args.as_slice().get(1) {
        let filepath = Path::new(&arg_path).to_path_buf();

        let _vec = read_to_vec(filepath)?;
        let _vec2 = _vec.clone();
        let _vec_len = _vec.len();

        let results_a = part_a(_vec);
        if results_a > 0 {
            println!("part A: found {} valid passwords out of {}", results_a, _vec_len);
        } else {
            println!("part A: found no valid passwords!");
        }

        let results_b = part_b(_vec2);
        if results_b > 0 {
            println!("part B: found {} valid passwords out of {}", results_b, _vec_len);
        } else {
            println!("part B: found no valid passwords!");
        }
    }

    Ok(())
}
