use std::collections::HashSet;

advent_of_code::solution!(20);

type Point = (isize, isize);
fn solve_part_one(input: &str, required_saved_time: usize) -> usize {
    let (rows, cols, value_map, wall_set) = shared_part(input);
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if wall_set.contains(&(r as isize, c as isize)) {
                continue;
            }
            for (new_r, new_c) in [(r + 2, c), (r + 1, c + 1), (r, c + 2), (r - 1, c + 1)] {
                if !(0..rows as isize).contains(&(new_r as isize))
                    || !(0..cols as isize).contains(&(new_c as isize))
                {
                    continue;
                }
                if wall_set.contains(&(new_r as isize, new_c as isize)) {
                    continue;
                }
                if value_map[r][c].abs_diff(value_map[new_r][new_c]) >= required_saved_time + 2 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn shared_part(input: &str) -> (usize, usize, Vec<Vec<isize>>, HashSet<(isize, isize)>) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut start: Point = (-1, -1);
    let mut end: Point = (-1, -1);
    let mut value_map: Vec<Vec<isize>> = Vec::new();
    let mut wall_set = HashSet::new();
    for (r, row) in input.lines().enumerate() {
        let mut row_vec = Vec::new();
        for (c, char) in row.chars().enumerate() {
            if char == 'S' {
                start = (r as isize, c as isize);
            }
            if char == 'E' {
                end = (r as isize, c as isize);
            }
            if char == '#' {
                wall_set.insert((r as isize, c as isize));
            }
            row_vec.push(-1);
        }
        value_map.push(row_vec);
    }
    value_map[start.0 as usize][start.1 as usize] = 0;
    let mut r = start.0;
    let mut c = start.1;
    while (r, c) != end {
        for (new_r, new_c) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            if !(0..rows as isize).contains(&new_r) || !(0..cols as isize).contains(&new_c) {
                continue;
            }
            if wall_set.contains(&(new_r, new_c)) {
                continue;
            }
            if value_map[new_r as usize][new_c as usize] != -1 {
                continue;
            }
            value_map[new_r as usize][new_c as usize] = value_map[r as usize][c as usize] + 1;
            r = new_r;
            c = new_c;
        }
    }
    (rows, cols, value_map, wall_set)
}

fn solve_part_two(input: &str, required_saved_time: usize) -> usize {
    let (rows, cols, value_map, wall_set) = shared_part(input);
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            let r_isize = r as isize;
            let c_isize = c as isize;
            if wall_set.contains(&(r_isize, c_isize)) {
                continue;
            }
            for radius in 2..=20 {
                for row_distance in 0..=radius {
                    let col_distance = radius - row_distance;
                    for (new_r, new_c) in HashSet::from([
                        (r_isize + row_distance, c_isize + col_distance),
                        (r_isize + row_distance, c_isize - col_distance),
                        (r_isize - row_distance, c_isize + col_distance),
                        (r_isize - row_distance, c_isize - col_distance),
                    ]) {
                        if !(0..rows as isize).contains(&(new_r))
                            || !(0..cols as isize).contains(&(new_c))
                        {
                            continue;
                        }
                        if wall_set.contains(&(new_r, new_c)) {
                            continue;
                        }
                        if value_map[r][c] - value_map[new_r as usize][new_c as usize]
                            >= (required_saved_time as isize + radius)
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve_part_one(input, 100))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve_part_two(input, 100))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file;

    #[test]
    fn test_part_one_case_1() {
        let result = solve_part_one(&read_file("examples", DAY), 2);
        assert_eq!(result, 44);
    }

    #[test]
    fn test_part_one_case_2() {
        let result = solve_part_one(&read_file("examples", DAY), 64);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_case_1() {
        let result = solve_part_two(&read_file("examples", DAY), 50);
        assert_eq!(result, 285);
    }

    #[test]
    fn test_part_two_case_2() {
        let result = solve_part_two(&read_file("examples", DAY), 74);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_two_case_3() {
        let result = solve_part_two(&read_file("examples", DAY), 76);
        assert_eq!(result, 3);
    }
}
