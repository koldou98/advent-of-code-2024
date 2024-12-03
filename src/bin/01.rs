use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut left = vec![];
    let mut right = vec![];
    input.lines().for_each(|line| {
        let content = line.split_once("   ").unwrap();
        left.push(content.0.parse::<i32>().unwrap());
        let val = content.1.parse::<i32>().unwrap();
        right.push(val);
    });
    left.sort();
    right.sort();

    let result = left.iter().zip(right).map(|(l, r)| (l - r).abs()).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        let content = line.split_once("   ").unwrap();
        map.entry(content.0.parse::<u32>().unwrap())
            .and_modify(|(l, _)| *l += 1)
            .or_insert((1, 0));
        map.entry(content.1.parse::<u32>().unwrap())
            .and_modify(|(_, r)| *r += 1)
            .or_insert((0, 1));
    });
    Some(map.iter().map(|(k, (l, r))| k * r * l).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
