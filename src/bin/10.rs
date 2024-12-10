use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut point_list = Vec::new();
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();
    let map: Vec<Vec<isize>> = parse_input(input, &mut point_list);
    let mut result = 0;
    let mut seen = HashSet::new();

    for point in point_list {
        seen.clear();
        let mut points_to_check = VecDeque::from([point]);
        while !points_to_check.is_empty() {
            let point = points_to_check.pop_front().unwrap();
            if seen.contains(&point) {
                continue;
            }
            seen.insert(point);
            if map[point.0 as usize][point.1 as usize] == 9 {
                result += 1;
            } else {
                for (y1, x1) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                    let new_y = point.0 + y1;
                    let new_x = point.1 + x1;

                    if in_bounds(point, &map, max_x, max_y, new_y, new_x) {
                        points_to_check.push_back((new_y, new_x));
                    }
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut point_list = Vec::new();
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();
    let map: Vec<Vec<isize>> = parse_input(input, &mut point_list);
    let mut result = 0;
    let mut memo = HashMap::new();
    for point in point_list {
        result += find_paths(point, &map, &mut memo, max_y, max_x);
    }
    Some(result)
}

fn find_paths(
    point: (isize, isize),
    map: &Vec<Vec<isize>>,
    memo: &mut HashMap<(isize, isize), u32>,
    max_y: usize,
    max_x: usize,
) -> u32 {
    if map[point.0 as usize][point.1 as usize] == 9 {
        return 1;
    }
    if memo.contains_key(&point) {
        return memo[&point];
    }
    let mut result = 0;
    for (y1, x1) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let new_y = point.0 + y1;
        let new_x = point.1 + x1;

        if in_bounds(point, map, max_y, max_x, new_y, new_x) {
            result += find_paths((new_y, new_x), map, memo, max_y, max_x);
        }
    }
    memo.insert(point, result);
    result
}

fn parse_input(input: &str, starting_positions: &mut Vec<(isize, isize)>) -> Vec<Vec<isize>> {
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(y, line)| {
            let mut vec = Vec::new();
            line.chars().enumerate().for_each(|(x, ch)| {
                let num = ch.to_digit(10).unwrap();
                vec.push(num as isize);
                if num == 0 {
                    starting_positions.push((y as isize, x as isize));
                }
            });
            vec
        })
        .collect()
}

fn in_bounds(
    point: (isize, isize),
    map: &[Vec<isize>],
    max_y: usize,
    max_x: usize,
    new_y: isize,
    new_x: isize,
) -> bool {
    (0..max_y as isize).contains(&new_y)
        && (0..max_x as isize).contains(&new_x)
        && map[new_y as usize][new_x as usize] == map[point.0 as usize][point.1 as usize] + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
