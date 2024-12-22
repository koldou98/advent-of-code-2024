use std::collections::{HashMap, HashSet, VecDeque};

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

pub fn part_two(input: &str) -> Option<isize> {
    let mut sequence_total = HashMap::new();
    for line in input.lines() {
        let mut num = line.parse::<usize>().unwrap();
        let mut buyer = VecDeque::from([num % 10]);
        let mut seen = HashSet::new();
        for _ in 0..2000 {
            num = calculate_step(num);
            buyer.push_back(num % 10);
            if buyer.len() >= 5 {
                let slice = (
                    buyer[0] as isize,
                    buyer[1] as isize,
                    buyer[2] as isize,
                    buyer[3] as isize,
                    buyer[4] as isize,
                );
                buyer.pop_front();
                let sequences = (
                    slice.1 - slice.0,
                    slice.2 - slice.1,
                    slice.3 - slice.2,
                    slice.4 - slice.3,
                );
                if seen.contains(&sequences) {
                    continue;
                }
                seen.insert(sequences);
                let sequence_last_value = slice.4;
                sequence_total
                    .entry(sequences)
                    .and_modify(|e| *e += sequence_last_value)
                    .or_insert(sequence_last_value);
            }
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
