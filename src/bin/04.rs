use crate::Search::{
    Backwards, DiagonalBottomLeft, DiagonalBottomRight, DiagonalTopLeft, DiagonalTopRight, Front,
    VerticalBottom, VerticalTop,
};

advent_of_code::solution!(4);

enum Search {
    Front,
    Backwards,
    VerticalTop,
    VerticalBottom,
    DiagonalTopRight,
    DiagonalTopLeft,
    DiagonalBottomRight,
    DiagonalBottomLeft,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let matrix = create_matrix(input);
    let height = matrix.len();
    let width = matrix[0].len();
    for y in 0..height {
        for x in 0..width {
            if matrix[y][x] == 'X' {
                result += find_xmas(&matrix, height, width, y, x);
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let matrix = create_matrix(input);
    let h = matrix.len();
    let w = matrix[0].len();
    for y in 0..h {
        for x in 0..w {
            if matrix[y][x] == 'A' && x > 0 && x < w - 1 && y > 0 && y < h - 1 {
                let d1 = [matrix[y - 1][x - 1], matrix[y + 1][x + 1]];
                let d2 = [matrix[y + 1][x - 1], matrix[y - 1][x + 1]];
                if check_diagonal(d1) && check_diagonal(d2) {
                    result += 1;
                }
            }
        }
    }
    Some(result)
}

fn find_xmas(matrix: &[Vec<char>], height: usize, width: usize, y: usize, x: usize) -> u32 {
    let mut result = 0;
    if x >= 3 {
        result += search(matrix, y, x, Backwards);
    }
    if width - x > 3 {
        result += search(matrix, y, x, Front);
    }
    if y >= 3 {
        result += search(matrix, y, x, VerticalTop);
    }
    if height - y > 3 {
        result += search(matrix, y, x, VerticalBottom);
    }
    if x >= 3 && y >= 3 {
        result += search(matrix, y, x, DiagonalTopLeft);
    }
    if width - x > 3 && y >= 3 {
        result += search(matrix, y, x, DiagonalTopRight);
    }
    if x >= 3 && height - y > 3 {
        result += search(matrix, y, x, DiagonalBottomLeft);
    }
    if width - x > 3 && height - y > 3 {
        result += search(matrix, y, x, DiagonalBottomRight);
    }
    result
}

fn search(matrix: &[Vec<char>], y: usize, x: usize, search_type: Search) -> u32 {
    let target = ['X', 'M', 'A', 'S'];
    let target_chars = target.iter().enumerate().skip(1);
    match search_type {
        Backwards => {
            for (i, char) in target_chars {
                if matrix[y][x - i] != *char {
                    return 0;
                }
            }
            1
        }
        Front => {
            for (i, char) in target_chars {
                if matrix[y][x + i] != *char {
                    return 0;
                }
            }
            1
        }
        VerticalTop => {
            for (i, char) in target_chars {
                if matrix[y - i][x] != *char {
                    return 0;
                }
            }
            1
        }
        VerticalBottom => {
            for (i, char) in target_chars {
                if matrix[y + i][x] != *char {
                    return 0;
                }
            }
            1
        }
        DiagonalTopLeft => {
            for (i, char) in target_chars {
                if matrix[y - i][x - i] != *char {
                    return 0;
                }
            }
            1
        }
        DiagonalTopRight => {
            for (i, char) in target_chars {
                if matrix[y - i][x + i] != *char {
                    return 0;
                }
            }
            1
        }
        DiagonalBottomLeft => {
            for (i, char) in target_chars {
                if matrix[y + i][x - i] != *char {
                    return 0;
                }
            }
            1
        }
        DiagonalBottomRight => {
            for (i, char) in target_chars {
                if matrix[y + i][x + i] != *char {
                    return 0;
                }
            }
            1
        }
    }
}

fn create_matrix(input: &str) -> Vec<Vec<char>> {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    matrix
}

fn check_diagonal(diagonal: [char; 2]) -> bool {
    diagonal == ['S', 'M'] || diagonal == ['M', 'S']
}

#[allow(dead_code)]
// Function to iterate over all the possible combinations of an element
fn find_xmas_loop(matrix: &[Vec<char>], height: usize, width: usize, y: usize, x: usize) -> u32 {
    let mut result = 0;
    let directions: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];
    let target = ['X', 'M', 'A', 'S'];

    for direction in directions {
        let mut valid = true;
        if x as isize + direction.0 * 3 < 0
            || x as isize + direction.0 * 3 >= width as isize
            || y as isize + direction.1 * 3 < 0
            || y as isize + direction.1 * 3 >= height as isize
        {
            continue;
        }

        for (i, char) in target.iter().enumerate().skip(1) {
            let new_x = direction.0 * i as isize + x as isize;
            let new_y = direction.1 * i as isize + y as isize;
            if matrix[new_y as usize][new_x as usize] != *char {
                valid = false;
                break;
            }
        }
        if valid {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
