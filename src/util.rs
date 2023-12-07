#[allow(unused_features)]
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::Chars;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(Path::new(path)).expect(&format!("could not read file at {:?}", path))
}

pub fn split_lines(s: &str) -> Vec<&str> {
    s.split("\n").collect()
}

pub fn char_at(str: &str, i: usize) -> char {
    str.chars().nth(i).unwrap()
}

pub fn char_at_opt(str: &str, i: i32) -> Option<char> {
    str.chars().nth(i as usize)
}

pub fn log_debug(debugable: &impl Debug) {
    println!("{}", &format!("{:?}", debugable));
}

pub fn int(string: &str) -> i32 {
    string
        .parse::<i32>()
        .expect(format!("Could not parse {:?} to i32", string).as_str())
}

pub fn ints(strings: Vec<&str>) -> Vec<i32> {
    strings.into_iter().map(int).collect()
}

pub fn ints_c(chars: Chars) -> Vec<i32> {
    let radix = 10;
    chars
        .into_iter()
        .map(|c| c.to_digit(radix).expect("could not convert char to i32") as i32)
        .collect()
}

pub fn int_big(string: &str) -> i64 {
    match string.parse::<i64>() {
        Ok(number) => number,
        Err(_) => panic!("Could not parse {:?} to i64", string),
    }
}

pub fn get_index((x, y): (i32, i32), n_cols: i32) -> i32 {
    (y * n_cols) + x
}

pub fn count_in_vec<T: Eq>(v: &Vec<T>, item: T) -> i32 {
    v.into_iter().filter(|i| item.eq(i)).count() as i32
}

pub fn print_ret<T: Debug>(v: T) -> T {
    log_debug(&v);
    v
}

pub fn log_all_items<T: Debug>(v: &Vec<T>) {
    for i in 0..v.len() {
        log_debug(&v.get(i).unwrap());
    }
}

fn get_coords(index: i32, width: i32) -> (i32, i32) {
    let x = index % width;
    (x, (index - x) / width)
}
