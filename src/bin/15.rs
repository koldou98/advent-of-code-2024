use std::collections::{HashSet, VecDeque};
use std::vec;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let mut position = (0, 0);
    let (mut map, movements) = parse_input_part_one(input.trim_end(), &mut position);
    map[position.0][position.1] = '.';
    for movement in movements.trim().chars() {
        match movement {
            '^' => {
                let mut b = None;
                for r in (0..=position.0 - 1).rev() {
                    let x = map[r][position.1];
                    match x {
                        'O' => {
                            if b.is_none() {
                                b = Some(r);
                            }
                        }
                        '#' => break,

                        '.' => {
                            if let Some(br) = b {
                                map[r][position.1] = 'O';
                                map[position.0][position.1] = '.';
                                map[br][position.1] = '.';
                            }
                            position = (position.0 - 1, position.1);
                            break;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
            }
            'v' => {
                let mut b = None;
                for r in position.0 + 1..map.len() {
                    match map[r][position.1] {
                        'O' => {
                            if b.is_none() {
                                b = Some(r);
                            }
                        }
                        '#' => break,
                        '.' => {
                            apply_row_box_movement_part_one(&mut position, &mut map, &mut b, r);
                            position = (position.0 + 1, position.1);
                            break;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
            }
            '>' => {
                let mut b = None;
                for c in position.1 + 1..map[0].len() {
                    match map[position.0][c] {
                        'O' => {
                            if b.is_none() {
                                b = Some(c);
                            }
                        }
                        '#' => {
                            break;
                        }
                        '.' => {
                            apply_column_box_movement_part_one(&mut position, &mut map, &mut b, c);
                            position = (position.0, position.1 + 1);
                            break;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
            }

            '<' => {
                let mut b = None;
                for c in (0..=position.1 - 1).rev() {
                    match map[position.0][c] {
                        'O' => {
                            if b.is_none() {
                                b = Some(c);
                            }
                        }
                        '#' => {
                            break;
                        }
                        '.' => {
                            apply_column_box_movement_part_one(&mut position, &mut map, &mut b, c);
                            position = (position.0, position.1 - 1);
                            break;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    let result = calculate_result(&mut map, 'O');

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut position = (0, 0);
    let (mut map, movements) = parse_input_part_two(input.trim_end(), &mut position);
    map[position.0][position.1] = '.';
    for movement in movements.trim().chars() {
        match movement {
            '^' => {
                let mut seen = HashSet::new();
                let mut can_be_pushed = true;
                for r in (0..=position.0 - 1).rev() {
                    if !can_be_pushed {
                        break;
                    }
                    match map[r][position.1] {
                        '#' => break,
                        ']' => {
                            can_be_pushed =
                                can_boxes_be_moved(r, position.1 - 1, &mut seen, &map, -1)
                        }
                        '[' => {
                            can_be_pushed = can_boxes_be_moved(r, position.1, &mut seen, &map, -1)
                        }
                        '.' => {
                            if can_be_pushed {
                                let mut vec: Vec<&(usize, usize)> = seen.iter().collect();
                                vec.sort();
                                for (r, c) in vec.iter() {
                                    map[*r][*c] = '.';
                                    map[*r][*c + 1] = '.';

                                    map[*r - 1][*c] = '[';
                                    map[*r - 1][*c + 1] = ']';
                                }
                                position = (position.0 - 1, position.1);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            'v' => {
                let mut seen = HashSet::new();
                let mut can_be_pushed = true;
                for r in position.0 + 1..map.len() {
                    if !can_be_pushed {
                        break;
                    }
                    match map[r][position.1] {
                        '#' => break,
                        ']' => {
                            can_be_pushed =
                                can_boxes_be_moved(r, position.1 - 1, &mut seen, &map, 1)
                        }
                        '[' => {
                            can_be_pushed = can_boxes_be_moved(r, position.1, &mut seen, &map, 1)
                        }
                        '.' => {
                            if can_be_pushed {
                                let mut vec: Vec<&(usize, usize)> = seen.iter().collect();
                                vec.sort();
                                for (r, c) in vec.iter().rev() {
                                    map[*r][*c] = '.';
                                    map[*r][*c + 1] = '.';
                                    map[*r + 1][*c] = '[';
                                    map[*r + 1][*c + 1] = ']';
                                }
                                position = (position.0 + 1, position.1);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            '>' => {
                let mut boxes = Vec::new();

                for c in position.1 + 1..map[0].len() {
                    match map[position.0][c] {
                        '[' => {
                            boxes.push((position.0, c));
                        }
                        '#' => {
                            break;
                        }
                        '.' => {
                            boxes.iter().rev().for_each(|(r, c)| {
                                map[*r][*c + 1] = '[';
                                map[*r][*c + 2] = ']';
                                map[*r][*c] = '.';
                            });
                            position = (position.0, position.1 + 1);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            '<' => {
                let mut boxes = Vec::new();
                for c in (0..=position.1 - 1).rev() {
                    match map[position.0][c] {
                        '[' => {
                            boxes.push((position.0, c));
                        }
                        '#' => {
                            break;
                        }
                        '.' => {
                            boxes.iter().rev().for_each(|(r, c)| {
                                map[*r][*c - 1] = '[';
                                map[*r][*c] = ']';
                                map[*r][*c + 1] = '.';
                            });
                            position = (position.0, position.1 - 1);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    let result = calculate_result(&mut map, '[');
    Some(result)
}

fn calculate_result(map: &mut [Vec<char>], box_char: char) -> usize {
    let mut result = 0;
    for (r, _) in map.iter().enumerate() {
        for (c, el) in map[r].iter().enumerate() {
            if *el == box_char {
                result += r * 100 + c;
            }
        }
    }
    result
}

fn can_boxes_be_moved(
    row: usize,
    col: usize,
    seen: &mut HashSet<(usize, usize)>,
    map: &[Vec<char>],
    direction: isize,
) -> bool {
    let mut boxes = VecDeque::from([(row, col)]);
    while !boxes.is_empty() {
        let (r, col) = boxes.pop_front().unwrap();
        let next_r = r as isize + direction;
        if seen.contains(&(row, col)) {
            continue;
        }
        seen.insert((row, col));
        let has_next_left = match map[next_r as usize][col] {
            '#' => false,
            ']' => can_boxes_be_moved(next_r as usize, col - 1, seen, map, direction),
            '[' => can_boxes_be_moved(next_r as usize, col, seen, map, direction),
            _ => true,
        };
        if !has_next_left {
            return false;
        }
        let has_next_right = match map[next_r as usize][col + 1] {
            '#' => false,
            ']' => can_boxes_be_moved(next_r as usize, col, seen, map, direction),
            '[' => can_boxes_be_moved(next_r as usize, col + 1, seen, map, direction),
            _ => true,
        };
        if !has_next_right {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
/// Function to print the map and the position
fn print_map(map: &[Vec<char>], position: (usize, usize)) {
    for (r, _) in map.iter().enumerate() {
        for (c, _) in map[r].iter().enumerate() {
            if r == position.0 && c == position.1 {
                print!("@")
            } else {
                print!("{}", map[r][c]);
            }
        }
        println!();
    }
}

fn parse_input_part_two(
    input: &str,
    initial_position: &mut (usize, usize),
) -> (Vec<Vec<char>>, String) {
    let (raw_map, movements) = input.split_once("\n\n").unwrap();
    let map: Vec<Vec<char>> = raw_map
        .lines()
        .enumerate()
        .map(|(r, row)| {
            let mut new_row = Vec::new();
            row.chars().enumerate().for_each(|(c, col)| match col {
                'O' => new_row.append(&mut vec!['[', ']']),
                '#' => new_row.append(&mut vec!['#', '#']),
                '@' => {
                    *initial_position = (r, 2 * c);
                    new_row.append(&mut vec!['@', '.']);
                }
                _ => new_row.append(&mut vec!['.', '.']),
            });
            new_row
        })
        .collect();
    let movements = movements.lines().collect::<String>();
    (map, movements)
}
fn parse_input_part_one(
    input: &str,
    initial_position: &mut (usize, usize),
) -> (Vec<Vec<char>>, String) {
    let (raw_map, movements) = input.split_once("\n\n").unwrap();
    let map: Vec<Vec<char>> = raw_map
        .lines()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, col)| {
                    if col == '@' {
                        *initial_position = (r, c)
                    }
                    col
                })
                .collect::<Vec<char>>()
        })
        .collect();
    let movements = movements.lines().collect::<String>();
    (map, movements)
}

fn apply_row_box_movement_part_one(
    position: &mut (usize, usize),
    map: &mut [Vec<char>],
    b: &mut Option<usize>,
    r: usize,
) {
    if let Some(br) = b {
        map[r][position.1] = 'O';
        map[position.0][position.1] = '.';
        map[*br][position.1] = '.';
    }
}

fn apply_column_box_movement_part_one(
    position: &mut (usize, usize),
    map: &mut [Vec<char>],
    b: &mut Option<usize>,
    c: usize,
) {
    if let Some(bc) = b {
        map[position.0][c] = 'O';
        map[position.0][position.1] = '.';
        map[position.0][*bc] = '.';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
