use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut starting_locations = Vec::new();
    let map: Vec<Vec<isize>> = parse_input(input, &mut starting_locations);
    let mut trailheads = Vec::new();
    for point in starting_locations {
        let hills = get_trailheads(&map, point);
        trailheads.push(hills);
    }
    let result = trailheads
        .iter()
        .map(|trailhead| trailhead.iter().collect::<HashSet<_>>().len() as u32)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut starting_locations = Vec::new();
    let map: Vec<Vec<isize>> = parse_input(input, &mut starting_locations);
    let mut trailheads = Vec::new();
    for point in starting_locations {
        let hills = get_trailheads(&map, point);
        trailheads.push(hills);
    }
    let result = trailheads
        .iter()
        .map(|trailhead| trailhead.len() as u32)
        .sum();
    Some(result)
}

fn get_trailheads(map: &[Vec<isize>], point: (isize, isize)) -> Vec<(isize, isize)> {
    let max_y = map.len();
    let max_x = map[0].len();
    let mut hills = Vec::new();
    let mut points_to_check = VecDeque::from([point]);
    while !points_to_check.is_empty() {
        let point = points_to_check.pop_front().unwrap();
        if map[point.0 as usize][point.1 as usize] == 9 {
            hills.push(point);
        } else {
            for (y1, x1) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let new_y = point.0 + y1;
                let new_x = point.1 + x1;
                if in_bounds(point, map, max_x, max_y, new_y, new_x) {
                    points_to_check.push_back((new_y, new_x));
                }
            }
        }
    }
    hills
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
