use std::collections::VecDeque;

use crate::util::{read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a("src/input/day_04_short.txt"), 13);
    assert_eq!(get_a("src/input/day_04.txt"), 24848)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/day_04_b_short.txt"), 30);
    assert_eq!(get_b("src/input/day_04.txt"), 7258152)
}

struct Card {
    my_numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
    id: String,
}

fn parse_card(line: &str) -> Card {
    let mut i = line.split(": ");
    let id = i.next().unwrap().to_string();
    let mut numbers_part = i.next().unwrap().split(" | ");

    let winning_part = numbers_part.next().unwrap();
    let mut winning_numbers = vec![];
    for winning_number in winning_part.split(" ") {
        if let Ok(number) = winning_number.parse::<i32>() {
            winning_numbers.push(number)
        }
    }

    let my_part = numbers_part.next().unwrap();
    let mut my_numbers = vec![];
    for my_number in my_part.split(" ") {
        if let Ok(number) = my_number.parse::<i32>() {
            my_numbers.push(number)
        }
    }

    Card {
        id,
        my_numbers,
        winning_numbers,
    }
}

fn count_score(card: Card) -> i32 {
    let matches = card.my_numbers.into_iter().fold(0, |points, number| {
        if card.winning_numbers.contains(&number) {
            return points + 1;
        }
        points
    });

    if matches == 0 {
        return 0;
    }

    2_i32.pow(matches - 1)
}

fn count_winning_numbers(card: &Card) -> i32 {
    card.my_numbers
        .clone()
        .into_iter()
        .fold(0, |points, number| {
            if card.winning_numbers.contains(&number) {
                return points + 1;
            }
            points
        })
}

pub fn get_a(file: &str) -> i32 {
    split_lines(read_file(file).as_str().trim())
        .into_iter()
        .map(parse_card)
        .map(count_score)
        .sum()
}

pub fn get_b(file: &str) -> i32 {
    let cards = split_lines(read_file(file).as_str().trim())
        .into_iter()
        .map(parse_card)
        .collect::<Vec<Card>>();

    let mut card_stack = VecDeque::from((1..cards.len() + 1).into_iter().collect::<Vec<usize>>());
    let mut count = 0;

    while let Some(card_id) = card_stack.pop_front() {
        count += 1;

        let card = cards.get(card_id - 1).unwrap();

        let winning_numbers = count_winning_numbers(card);

        if winning_numbers == 0 {
            continue;
        }

        let next_card = card_id + 1;
        let won_cards = next_card..(next_card + (winning_numbers as usize));

        for won_card in won_cards {
            card_stack.push_back(won_card);
        }
    }

    count
}
