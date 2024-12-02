use crate::Trend::{Dec, Inc};
use std::cmp::PartialEq;

advent_of_code::solution!(2);

#[derive(PartialEq)]
enum Trend {
    Inc,
    Dec,
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<i8> = line
                .split_whitespace()
                .map(|x| x.parse::<i8>().unwrap())
                .collect();
            is_valid_combination(split)
        })
        .filter(|&x| x)
        .count();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split: Vec<i8> = line
                .split_whitespace()
                .map(|x| x.parse::<i8>().unwrap())
                .collect();
            let combinations = produce_combinations(split);
            for combination in combinations {
                if is_valid_combination(combination) {
                    return true;
                }
            }
            false
        })
        .filter(|&x| x)
        .count();
    Some(result)
}

fn is_valid_combination(combination: Vec<i8>) -> bool {
    let mut split = combination.into_iter();
    let first: i8 = split.next().unwrap();
    let mut trend = None;
    let mut prev = first;
    for val in split {
        let val: i8 = val;
        if trend.is_none() {
            trend = calculate_trend(prev, val);
        }
        if is_not_valid(prev, val) || calculate_trend(prev, val) != trend {
            return false;
        }

        prev = val;
    }
    true
}

fn calculate_trend(first: i8, second: i8) -> Option<Trend> {
    if second > first {
        Some(Inc)
    } else {
        Some(Dec)
    }
}

fn is_not_valid(a: i8, b: i8) -> bool {
    let mut result = false;
    if (a - b).abs() < 1 || (a - b).abs() > 3 {
        result = true;
    }
    result
}

fn produce_combinations(ints: Vec<i8>) -> Vec<Vec<i8>> {
    let mut combinations = vec![];
    for i in 0..ints.len() {
        let mut vec = ints.clone();
        vec.remove(i);
        combinations.push(vec)
    }
    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
