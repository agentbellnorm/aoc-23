use crate::util::{char_at, read_file, split_lines};

#[test]
fn a() {
    assert_eq!(get_a("src/input/day_03_short.txt"), 4361);
    assert_eq!(get_a("src/input/day_03.txt"), 521601)
}

#[test]
fn b() {
    assert_eq!(get_b("src/input/day_03_short.txt"), 467835);
    assert_eq!(get_b("src/input/day_03.txt"), 80694070)
}

fn in_bounds(min: i32, x: i32, max: i32) -> bool {
    x >= min && x < max
}

#[derive(Debug)]
struct PartNumber {
    as_string: String,
    coords: Vec<[i32; 2]>,
}

fn get_adjacent(coords: &Vec<[i32; 2]>, [max_x, max_y]: [i32; 2]) -> Vec<[i32; 2]> {
    coords
        .iter()
        .flat_map(|&[x, y]| {
            vec![
                [x, y + 1],
                [x, y - 1],
                [x + 1, y],
                [x - 1, y],
                [x + 1, y + 1],
                [x + 1, y - 1],
                [x - 1, y + 1],
                [x - 1, y - 1],
            ]
        })
        .filter(|&[x, y]| {
            !coords.contains(&[x, y]) && in_bounds(0, x, max_x) && in_bounds(0, y, max_y)
        })
        .collect::<Vec<[i32; 2]>>()
}

fn get_part_nubmers(lines: &Vec<&str>) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = vec![];

    let mut current_part_number: Option<PartNumber> = None;

    for (y, row) in lines.iter().enumerate() {
        for (x, current_char) in row.chars().enumerate() {
            if current_char.is_numeric() {
                match current_part_number {
                    Some(ref mut part_number) => {
                        part_number.as_string.push(current_char);
                        part_number.coords.push([x as i32, y as i32])
                    }
                    None => {
                        current_part_number = Some(PartNumber {
                            as_string: String::from(current_char),
                            coords: vec![[x as i32, y as i32]],
                        })
                    }
                }
            } else {
                if let Some(part_number) = current_part_number {
                    part_numbers.push(part_number);
                    current_part_number = None;
                }
            }
        }
    }

    part_numbers
}

pub fn get_a(file: &str) -> i32 {
    let f = read_file(file);
    let lines = split_lines(f.as_str().trim());
    let part_numbers = get_part_nubmers(&lines);

    let bounds = [lines[0].len() as i32, lines.len() as i32];

    part_numbers
        .into_iter()
        .filter(|part_number| {
            get_adjacent(&part_number.coords, bounds)
                .into_iter()
                .any(|[x, y]| {
                    let char = char_at(lines[y as usize], x as usize);
                    match char {
                        '.' => {
                            println!("char was {}", char);
                            false
                        }
                        _ => true,
                    }
                })
        })
        .map(|part_number| part_number.as_string.parse::<i32>().unwrap())
        .sum()
}

pub fn get_b(file: &str) -> i32 {
    let f = read_file(file);
    let lines = split_lines(f.as_str().trim());
    let part_numbers = get_part_nubmers(&lines);
    let bounds = [lines[0].len() as i32, lines.len() as i32];

    let mut gear_ratio = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '*' {
                continue;
            }

            let adj_numbers = part_numbers
                .iter()
                .filter(|part| {
                    get_adjacent(&part.coords, bounds)
                        .into_iter()
                        .any(|[adj_x, adj_y]| adj_x == x as i32 && adj_y == y as i32)
                })
                .collect::<Vec<&PartNumber>>();
            if adj_numbers.len() == 2 {
                gear_ratio += adj_numbers[0].as_string.parse::<i32>().unwrap()
                    * adj_numbers[1].as_string.parse::<i32>().unwrap();
            }
        }
    }

    gear_ratio
}
