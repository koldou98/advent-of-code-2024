use std::collections::{HashMap, HashSet};

advent_of_code::solution!(19);

fn count_possible<'a>(
    pattern: &'a str,
    towels: &HashSet<&str>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(&c) = memo.get(pattern) {
        return c;
    }

    if pattern.trim().is_empty() {
        return 1;
    }

    let mut count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            count += count_possible(&pattern[towel.len()..], towels, memo);
        }
    }

    *memo.entry(pattern).or_insert(0) += count;
    count
}

fn parse_input(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let mut it = input.lines();

    let towels = it.next().unwrap().split(", ").collect();

    _ = it.next();

    let patterns = it.collect();

    (towels, patterns)
}
pub fn part_one(input: &str) -> Option<usize> {
    let (towels, patterns) = parse_input(input);
    let mut memo = HashMap::new();

    let result = patterns
        .iter()
        .map(|pattern| count_possible(&pattern, &towels, &mut memo))
        .filter(|pattern_count| pattern_count > &0)
        .collect::<Vec<_>>()
        .len();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, patterns) = parse_input(input);
    let mut memo = HashMap::new();

    let result = patterns
        .iter()
        .map(|pattern| count_possible(&pattern, &towels, &mut memo))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
