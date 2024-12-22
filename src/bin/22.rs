use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

const MODULO: usize = 16_777_216;

fn calculate_step(num: usize) -> usize {
    let num = (num ^ (num * 64)) % MODULO;
    let num = (num ^ (num / 32)) % MODULO;
    let num = (num ^ (num * 2048)) % MODULO;
    num
}
pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .map(|l| {
            let mut num = l.parse::<usize>().unwrap();
            for _ in 0..2000 {
                num = calculate_step(num);
            }
            num
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sequence_total = HashMap::new();
    for line in input.lines() {
        let mut num = line.parse::<usize>().unwrap();
        let mut buyer = Vec::from([num % 10]);
        for _ in 0..2000 {
            num = calculate_step(num);
            buyer.push(num % 10);
        }
        let mut seen = HashSet::new();
        for i in 0..buyer.len() - 4 {
            let slice = &buyer[i..i + 5];
            let mut sequences = Vec::new();
            for i in 1..slice.len() {
                sequences.push(slice[i] as isize - slice[i - 1] as isize)
            }
            let sequences = (sequences[0], sequences[1], sequences[2], sequences[3]);
            if seen.contains(&sequences) {
                continue;
            }
            seen.insert(sequences);
            let sequence_last_value = *slice.last().unwrap();
            sequence_total
                .entry(sequences)
                .and_modify(|e| *e += sequence_last_value)
                .or_insert(sequence_last_value);
        }
    }
    let result = sequence_total.values().max().unwrap();
    Some(*result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file_part;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(23));
    }
}
