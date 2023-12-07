use std::collections::HashMap;

use crate::util::{char_at, read_file, split_lines};

#[test]
fn a() {
    // assert_eq!(get_a("src/input/day_01_short.txt"), 142);
    // assert_eq!(get_a("src/input/day_01.txt"), 53334)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/day_01_b_short.txt"), 281);
    assert_eq!(get_b("src/input/day_01.txt"), 52834)
}

#[test]
fn first_and_last() {
    assert_eq!(get_first_and_last_digit("d2s8jlf3cio"), "23");
}

fn get_number(s: &str, index: usize) -> Option<i32> {
    let mut numbers: HashMap<String, i32> = HashMap::new();
    numbers.insert("1".to_string(), 1);
    numbers.insert("2".to_string(), 2);
    numbers.insert("3".to_string(), 3);
    numbers.insert("4".to_string(), 4);
    numbers.insert("5".to_string(), 5);
    numbers.insert("6".to_string(), 6);
    numbers.insert("7".to_string(), 7);
    numbers.insert("8".to_string(), 8);
    numbers.insert("9".to_string(), 9);
    numbers.insert("one".to_string(), 1);
    numbers.insert("two".to_string(), 2);
    numbers.insert("three".to_string(), 3);
    numbers.insert("four".to_string(), 4);
    numbers.insert("five".to_string(), 5);
    numbers.insert("six".to_string(), 6);
    numbers.insert("seven".to_string(), 7);
    numbers.insert("eight".to_string(), 8);
    numbers.insert("nine".to_string(), 9);

    for number in numbers.keys() {
        if s[index..s.len()].starts_with(number) {
            return Some(*numbers.get(number).unwrap());
        }
    }

    None
}

fn get_first_and_last_digit(s: &str) -> String {
    let mut numbers = vec![];

    for i in 0..s.len() {
        if let Some(number) = get_number(s, i) {
            numbers.push(number);
        }
    }

    format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
}

pub fn get_b(file: &str) -> i32 {
    split_lines(read_file(file).as_str().trim())
        .iter()
        .map(|line| get_first_and_last_digit(line).parse::<i32>().unwrap())
        .sum()
}

pub fn get_a(file: &str) -> i32 {
    split_lines(read_file(file).as_str().trim())
        .iter()
        .map(|line| get_first_and_last_digit(line).parse::<i32>().unwrap())
        .sum()
}
