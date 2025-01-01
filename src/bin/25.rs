use std::iter::zip;

advent_of_code::solution!(25);

fn transpose<T>(matrix: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if matrix.is_empty() {
        return vec![];
    }
    let cols = matrix[0].len();

    (0..cols)
        .map(|j| matrix.iter().map(|row| row[j].clone()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut keys: Vec<Vec<usize>> = Vec::new();
    let mut locks: Vec<Vec<usize>> = Vec::new();

    let block = input.split("\n\n");
    for block in block {
        let mut grid = Vec::new();
        for line in block.lines() {
            let line_values: Vec<char> = line.chars().collect();
            grid.push(line_values);
        }
        let transposed_grid = transpose(&grid);

        if transposed_grid[0][0] == '#' {
            locks.push(get_height(transposed_grid));
        } else {
            keys.push(get_height(transposed_grid));
        }
    }

    let mut total = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if zip(lock, key).all(|(l, k)| l + k <= 5) {
                total += 1;
            }
        }
    }
    Some(total)
}

fn get_height(transposed_grid: Vec<Vec<char>>) -> Vec<usize> {
    transposed_grid
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count() - 1)
        .collect()
}

pub fn part_two(_: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
