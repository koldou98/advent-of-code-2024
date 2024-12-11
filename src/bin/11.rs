use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim();
    let stones = input.split_whitespace().collect::<Vec<_>>();
    let mut memo = HashMap::new();
    let mut result = 0;
    for stone in stones {
        result += apply_rules(stone.to_string(), 25, &mut memo);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();
    let stones = input.split_whitespace().collect::<Vec<_>>();
    let mut memo = HashMap::new();
    let mut result = 0;
    for stone in stones {
        result += apply_rules(stone.to_string(), 75, &mut memo);
    }

    Some(result)
}

fn apply_rules<'a>(
    stone: String,
    times: usize,
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    if memo.contains_key(&(stone.clone(), times)) {
        return memo[&(stone, times)];
    }
    let result;
    if times == 0 {
        result = 1;
    } else if stone == "0" {
        result = apply_rules("1".to_string(), times - 1, memo);
    } else if stone.len() % 2 == 0 {
        let (p1, p2) = stone.split_at(stone.len() / 2);
        let p1 = p1.parse::<usize>().unwrap().to_string();
        let p2 = p2.parse::<usize>().unwrap().to_string();
        result = apply_rules(p1, times - 1, memo) + apply_rules(p2, times - 1, memo);
    } else {
        let new_stone = stone.parse::<usize>().unwrap() * 2024;
        result = apply_rules(new_stone.to_string(), times - 1, memo);
    }
    memo.insert((stone, times), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
