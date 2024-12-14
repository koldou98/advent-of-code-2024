use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

const CORNER_CHECKS: [(isize, isize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
const SINGLE_CORNERS: [[bool; 4]; 8] = [
    [true, false, false, false],
    [false, true, false, false],
    [false, false, true, false],
    [false, false, false, true],
    [true, true, true, false],
    [true, true, false, true],
    [true, false, true, true],
    [false, true, true, true],
];
const DOUBLE_CORNERS: [[bool; 4]; 2] = [[true, false, false, true], [false, true, true, false]];

pub fn part_one(input: &str) -> Option<usize> {
    let map: Vec<Vec<char>> = parse_input(input.trim());
    let rows = map.len();
    let columns = map[0].len();
    let mut visited = HashSet::new();
    let mut result = 0;
    for (i, col) in map.iter().enumerate() {
        for (j, _) in col.iter().enumerate() {
            let (perimeter, area) =
                get_area_and_perimeter(&map, (i, j), &mut visited, columns, rows);
            result += perimeter * area;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Vec<Vec<char>> = parse_input(input.trim());
    let rows = map.len();
    let columns = map[0].len();
    let mut visited = HashSet::new();
    let mut result = 0;
    for (i, col) in map.iter().enumerate() {
        for (j, _) in col.iter().enumerate() {
            if !visited.contains(&(i, j)) {
                let region = get_region(&map, (i, j), &mut visited, columns, rows);
                result += region.len() * get_region_sides(region);
            }
        }
    }
    Some(result)
}

fn get_region_sides(region: HashSet<(isize, isize)>) -> usize {
    let (mut min_r, mut max_r) = (isize::MAX, 0);
    let (mut min_c, mut max_c) = (isize::MAX, 0);

    for (r, c) in region.iter() {
        (min_r, max_r) = (min_r.min(*r), max_r.max(*r));
        (min_c, max_c) = (min_c.min(*c), max_c.max(*c));
    }
    let mut sides = 0;
    for r in min_r - 1..=max_r {
        for c in min_c - 1..=max_c {
            let mut corners = [false; 4];
            for (corners_idx, (offset_r, offset_c)) in CORNER_CHECKS.iter().enumerate() {
                corners[corners_idx] = region.contains(&(r + offset_r, c + offset_c));
            }

            if SINGLE_CORNERS.contains(&corners) {
                sides += 1;
            } else if DOUBLE_CORNERS.contains(&corners) {
                sides += 2;
            }
        }
    }
    sides
}

fn get_region(
    map: &[Vec<char>],
    point: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    columns: usize,
    rows: usize,
) -> HashSet<(isize, isize)> {
    let mut region = HashSet::from([(point.0 as isize, point.1 as isize)]);
    visited.insert(point);
    let mut region_plants = VecDeque::from([point]);
    while !region_plants.is_empty() {
        let plant = region_plants.pop_front().unwrap();
        for direction in DIRECTIONS {
            let next_row = plant.0 as isize + direction.0;
            let next_column = plant.1 as isize + direction.1;
            let new_point = (next_row as usize, next_column as usize);
            if in_bounds(rows, columns, next_row, next_column)
                && same_plant(map[plant.0][plant.1], map[new_point.0][new_point.1])
                && !visited.contains(&new_point)
            {
                visited.insert(new_point);
                region_plants.push_back(new_point);
                region.insert((next_row, next_column));
            }
        }
    }
    region
}

fn get_area_and_perimeter(
    map: &[Vec<char>],
    point: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    columns: usize,
    rows: usize,
) -> (usize, usize) {
    let mut area = 1;
    let mut sides = 4;
    if visited.contains(&point) {
        return (0, 0);
    }
    visited.insert(point);
    for to_check in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let next_row = point.0 as isize + to_check.0;
        let next_column = point.1 as isize + to_check.1;
        let new_point = (next_row as usize, next_column as usize);
        if in_bounds(rows, columns, next_row, next_column)
            && same_plant(map[point.0][point.1], map[new_point.0][new_point.1])
        {
            sides -= 1;
            let result = get_area_and_perimeter(map, new_point, visited, columns, rows);
            area += result.0;
            sides += result.1;
        }
    }
    (area, sides)
}

fn same_plant(actual: char, next: char) -> bool {
    actual == next
}

fn in_bounds(rows: usize, columns: usize, next_row: isize, next_column: isize) -> bool {
    (0..rows as isize).contains(&next_row) && (0..columns as isize).contains(&next_column)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }
    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(80));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }
    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }
    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }
    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }
}
