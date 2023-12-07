use regex::Regex;

use crate::util::{read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a("src/input/day_02_short.txt"), 8);
    assert_eq!(get_a("src/input/day_02.txt"), 2683)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/day_02_short.txt"), 2286);
    assert_eq!(get_b("src/input/day_02.txt"), 49710);
}

#[derive(Debug)]
struct Draw {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>,
}

struct Game {
    id: i32,
    draws: Vec<Draw>,
}

fn parse_game(line: &str) -> Game {
    let mut iter = line.split(": ").into_iter();
    let game_part = iter.next().unwrap();
    let game_id = game_part.split(" ").last().unwrap().parse::<i32>().unwrap();
    let draws = iter
        .next()
        .unwrap()
        .split("; ")
        .into_iter()
        .map(|raw_draw| {
            let mut draw = Draw {
                red: None,
                green: None,
                blue: None,
            };

            for color in raw_draw.split(", ") {
                let mut i = color.split(" ").into_iter();
                let n = i.next().unwrap().parse::<i32>().unwrap();
                match i.next().unwrap() {
                    "red" => draw.red = Some(n),
                    "green" => draw.green = Some(n),
                    "blue" => draw.blue = Some(n),
                    _ => panic!("couldnt parse color"),
                }
            }
            draw
        })
        .collect::<Vec<Draw>>();

    Game { id: game_id, draws }
}

pub fn get_a(file: &str) -> i32 {
    split_lines(read_file(file).as_str().trim())
        .into_iter()
        .map(parse_game)
        .filter(|game| {
            !game.draws.iter().any(|draw| {
                draw.red.unwrap_or(0) > 12
                    || draw.green.unwrap_or(0) > 13
                    || draw.blue.unwrap_or(0) > 14
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn get_b(file: &str) -> i32 {
    split_lines(read_file(file).as_str().trim())
        .into_iter()
        .map(parse_game)
        .map(|game| {
            let minimum_set = game.draws.into_iter().fold(
                Draw {
                    red: Some(i32::MIN),
                    green: Some(i32::MIN),
                    blue: Some(i32::MIN),
                },
                |acc, v| Draw {
                    red: Some(i32::max(acc.red.unwrap(), v.red.unwrap_or(i32::MIN))),
                    green: Some(i32::max(acc.green.unwrap(), v.green.unwrap_or(i32::MIN))),
                    blue: Some(i32::max(acc.blue.unwrap(), v.blue.unwrap_or(i32::MIN))),
                },
            );

            println!("minimum set for game {} is {:?}", game.id, minimum_set);

            minimum_set.red.unwrap() * minimum_set.green.unwrap() * minimum_set.blue.unwrap()
        })
        .sum()
}
