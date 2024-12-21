use std::collections::{HashMap, VecDeque};
use std::iter::zip;
use std::string::String;

advent_of_code::solution!(21);

/// Numeric Keypad:
/// ```text
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
const NUM_KEYPAD: [[&str; 3]; 4] = [
    ["7", "8", "9"],
    ["4", "5", "6"],
    ["1", "2", "3"],
    ["", "0", "A"],
];

/// Direction Keypad:
/// ```text
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
/// ```
const DIR_KEYPAD: [[&str; 3]; 2] = [["", "^", "A"], ["<", "v", ">"]];
type Point = (isize, isize);

fn map_keypad<const N: usize>(keypad: &[[&str; N]]) -> HashMap<String, Point> {
    let mut positions: HashMap<String, Point> = HashMap::new();

    for (r, row) in keypad.iter().enumerate() {
        for (c, str) in row.iter().enumerate() {
            if !str.is_empty() {
                positions.insert(str.to_string(), (r as isize, c as isize));
            }
        }
    }
    positions
}

fn calculate_keypad_sequences<const N: usize>(
    keypad: &[[&str; N]],
) -> HashMap<(String, String), Vec<String>> {
    let positions = map_keypad(keypad);
    let mut sequences: HashMap<(String, String), Vec<String>> = HashMap::new();
    for x in positions.keys() {
        for y in positions.keys() {
            if x == y {
                sequences.insert((x.clone(), y.clone()), Vec::from(["A".to_string()]));
                continue;
            }
            let mut possibilities = Vec::new();
            let mut optimal_movements = usize::MAX;
            let mut queue: VecDeque<(Point, String)> =
                VecDeque::from([(*positions.get(x).unwrap(), "".to_string())]);
            let mut optimal_found = false;
            while !queue.is_empty() {
                let ((r, c), movements) = queue.pop_front().unwrap();
                for (next_r, next_c, next_movement) in [
                    (r - 1, c, "^"),
                    (r + 1, c, "v"),
                    (r, c - 1, "<"),
                    (r, c + 1, ">"),
                ] {
                    if !(0..keypad.len() as isize).contains(&next_r)
                        || !(0..keypad[0].len() as isize).contains(&next_c)
                    {
                        continue;
                    }
                    let next_r_usize = next_r as usize;
                    let next_c_usize = next_c as usize;
                    if keypad[next_r_usize][next_c_usize].is_empty() {
                        continue;
                    }
                    if keypad[next_r_usize][next_c_usize] == *y {
                        let total_movements = movements.len() + 1;
                        if optimal_movements < total_movements {
                            optimal_found = true;
                            break;
                        }
                        optimal_movements = total_movements;
                        possibilities.push(format!("{movements}{next_movement}A"));
                    } else {
                        queue.push_back(((next_r, next_c), format!("{movements}{next_movement}")));
                    }
                }
                if optimal_found {
                    break;
                }
            }
            sequences.insert((x.clone(), y.clone()), possibilities);
        }
    }
    sequences
}

fn solve(line: &str, sequences: &HashMap<(String, String), Vec<String>>) -> Vec<String> {
    let mut options = Vec::new();
    let complete_code = format!("A{line}"); // Code starts at A
    zip(complete_code.chars(), line.chars()).for_each(|(x, y)| {
        options.push(
            sequences
                .get(&(x.to_string(), y.to_string()))
                .unwrap()
                .clone(),
        )
    });
    let parsed_options: Vec<String> = cartesian_product(&options)
        .iter()
        .map(|vec| vec.join(""))
        .collect();
    parsed_options
}

fn cartesian_product(options: &[Vec<String>]) -> Vec<Vec<String>> {
    if options.is_empty() {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    for item in options[0].iter() {
        for mut sub_product in cartesian_product(&options[1..]) {
            sub_product.insert(0, item.clone());
            result.push(sub_product);
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = calculate_complexity(input, 2);

    Some(result)
}

fn calculate_length<'a>(
    option: String,
    depth: usize,
    memo: &mut HashMap<(String, usize), usize>,
    dir_sequences: &'a HashMap<(String, String), Vec<String>>,
    dir_lengths: &'a HashMap<&'a (String, String), usize>,
) -> usize {
    if memo.contains_key(&(option.clone(), depth)) {
        return memo[&(option, depth)];
    }

    let string = format!("A{option}");
    let seq_zip = zip(string.chars(), option.chars());
    if depth == 1 {
        return seq_zip
            .map(|(x, y)| dir_lengths[&(x.to_string(), y.to_string())])
            .sum();
    }
    let mut length = 0;
    for (x, y) in seq_zip {
        length += dir_sequences
            .get(&(x.to_string(), y.to_string()))
            .unwrap()
            .iter()
            .map(|opt| calculate_length(opt.clone(), depth - 1, memo, dir_sequences, dir_lengths))
            .min()
            .unwrap();
    }
    memo.insert((option, depth), length);
    length
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = calculate_complexity(input, 25);

    Some(result)
}

fn calculate_complexity(input: &str, robots: usize) -> usize {
    let num_sequences = calculate_keypad_sequences(&NUM_KEYPAD);
    let mut memo: HashMap<(String, usize), usize> = HashMap::new();
    let dir_sequences = calculate_keypad_sequences(&DIR_KEYPAD);
    let dir_lengths: HashMap<&(String, String), usize> =
        dir_sequences.iter().map(|(k, v)| (k, v[0].len())).collect();

    let result = input
        .lines()
        .map(|line| {
            let inputs = solve(line, &num_sequences);
            let i = inputs
                .iter()
                .map(|seq| {
                    calculate_length(seq.clone(), robots, &mut memo, &dir_sequences, &dir_lengths)
                })
                .min()
                .unwrap();
            i * line[..line.len() - 1].parse::<usize>().unwrap()
        })
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
