use path::{Path, PathBuf};
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::{env, fs, path, str};
use text_io::scan;

#[derive(Clone, Debug)]
struct PassportField {
    key: String,
    val: String,
}
type Passport = HashMap<String, PassportField>;

fn read_in_chunks(path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = fs::File::open(path)?;
    let mut filebuf: Vec<u8> = Vec::new();
    file.read_to_end(&mut filebuf)?;
    let as_str = str::from_utf8(&filebuf)?;
    // split records to strings on a blank newline
    let as_chunks = as_str
        .split("\n\n")
        .enumerate()
        // remove spurious newlines inside a record
        .map(|(_, s)| s.to_owned().replace('\n', " ").to_string())
        .collect::<Vec<String>>();
    Ok(as_chunks)
}

fn is_hex(input: &str) -> Option<String> {
    let _chars = input[1..].chars();
    let ldr = input.starts_with("#");
    let maybe_hex = _chars
        .into_iter()
        .filter_map(|c| if c.is_digit(16) { Some(c) } else { None })
        .collect::<String>();
    if ldr && maybe_hex.len() == 6 {
        return Some(input.to_owned());
    }
    None
}

fn is_height(input: &str) -> Option<String> {
    let valid_sfx = vec!["in", "cm"];
    for sfx in valid_sfx {
        let split = input.split(&sfx);
        if let Ok(_nums) = split.take(1).collect::<String>().parse::<i32>() {
            if (sfx == "cm" && _nums >= 150 && _nums <= 193)
                || (sfx == "in" && _nums >= 59 && _nums <= 76)
            {
                return Some(input.to_owned());
            }
        }
    }
    None
}

#[allow(unused_assignments)]
fn parse_record(input: String) -> (String, String) {
    let mut key = String::new();
    let mut val = String::new();
    scan!(input.bytes() => "{}:{}", key, val);
    (key, val)
}

fn field_validator(field: &PassportField) -> Option<PassportField> {
    let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let key: &str = &field.key;
    let val: &str = &field.val;
    if let Ok(_num) = val.parse::<i32>() {
        let _num_len = val.chars().count();
        if (key == "pid" && _num_len == 9)
            || (key == "byr" && _num >= 1920 && _num <= 2002)
            || (key == "iyr" && _num >= 2010 && _num <= 2020)
            || (key == "eyr" && _num >= 2020 && _num <= 2030)
        {
            return Some(PassportField {
                key: key.to_owned(),
                val: val.to_owned(),
            });
        }
    } else if key == "hgt" {
        if let Some(_hgt) = is_height(&val) {
            return Some(PassportField {
                key: key.to_owned(),
                val: val.to_owned(),
            });
        }
    } else if key == "ecl" && valid_ecl.contains(&&val[..]) {
        return Some(PassportField {
            key: key.to_owned(),
            val: val.to_owned(),
        });
    } else if key == "hcl" {
        if let Some(_hex) = is_hex(val) {
            return Some(PassportField {
                key: key.to_owned(),
                val: val.to_owned(),
            });
        }
    }
    None
}

fn passport_solver(chunks: Vec<String>, validate: bool) -> Option<Vec<Passport>> {
    let mut valid_passports = Vec::new();
    // leave out the "cid" key - it's optional
    let valid_keys = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    for c in chunks.iter() {
        let mut passport: Passport = HashMap::new();
        let fields = c.split(" ").map(|s| s.to_owned());
        let _fields = fields.clone();
        for r in _fields {
            let record = parse_record(r);
            let _key: &str = &record.0;
            let _val: &str = &record.1;
            let _is_valid_key = valid_keys.contains(&_key);
            let field = PassportField {
                key: _key.to_owned(),
                val: _val.to_owned(),
            };
            if _is_valid_key && validate {
                if let Some(_field) = field_validator(&field) {
                    passport.insert(_key.to_owned(), _field);
                } else { break; }
            } else if _is_valid_key {
                passport.insert(_key.to_owned(), field);
            }
        }
        if passport.keys().count() == 7 {
            valid_passports.push(passport);
        }
    }
    if valid_passports.len() > 0 {
        return Some(valid_passports);
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg_path) = args.as_slice().get(1) {
        let filepath = Path::new(&arg_path).to_path_buf();

        let _vec = read_in_chunks(filepath)?;
        let _vec2 = _vec.clone();
        if let Some(result_a) = passport_solver(_vec, false) {
            println!("part A: found {} passports", result_a.len());
        } else {
            println!("part A: found no passports!");
        }
        if let Some(result_b) = passport_solver(_vec2, true) {
            println!("part B: found {} validated passports", result_b.len());
        } else {
            println!("part B: found no valid passports!");
        }
    }
    Ok(())
}
