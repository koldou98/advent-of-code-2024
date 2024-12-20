use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

advent_of_code::solution!(18);

fn part_one_solution(input: &str, bytes: usize, max_r: usize, max_c: usize) -> Option<usize> {
    let corrupted_bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            let (col, row) = l.split_once(',').unwrap();
            (row.parse().unwrap(), col.parse().unwrap())
        })
        .collect();
    let result: usize = find_len_dijkstra(&corrupted_bytes[..bytes], (max_r, max_c));
    Some(result)
}

fn find_len_dijkstra(bytes: &[(usize, usize)], end: (usize, usize)) -> usize {
    let mut priority_queue: BinaryHeap<Reverse<(usize, usize, usize)>> =
        BinaryHeap::from([Reverse((0, 0, 0))]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while !priority_queue.is_empty() {
        let (cost, r, c) = priority_queue.pop().unwrap().0;
        if (r, c) == end {
            return cost;
        }
        if visited.contains(&(r, c)) {
            continue;
        }
        visited.insert((r, c));
        for (dr, dc) in [(0, 1), (-1, 0), (0, -1), (1, 0)] {
            let r_isize = r as isize;
            let c_isize = c as isize;
            let next_r = r_isize + dr;
            let next_c = c_isize + dc;
            if (0..=end.0 as isize).contains(&next_r)
                && (0..=end.1 as isize).contains(&next_c)
                && !bytes.contains(&(next_c as usize, next_r as usize))
            {
                priority_queue.push(Reverse((cost + 1, next_r as usize, next_c as usize)));
            }
        }
    }
    0
}

fn part_two_solution(input: &str, max_c: usize, max_r: usize) -> Option<String> {
    let corrupted_bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            let (col, row) = l.split_once(',').unwrap();
            (row.parse().unwrap(), col.parse().unwrap())
        })
        .collect();
    let result = binary_search_corrupted_bytes(&corrupted_bytes, (max_c, max_r));
    let result = format!("{},{}", result.1, result.0);
    Some(result)
}

fn binary_search_corrupted_bytes(bytes: &[(usize, usize)], end: (usize, usize)) -> (usize, usize) {
    let mut low = 0;
    let mut high = bytes.len() - 1;
    while low < high {
        let mid = (low + high) / 2;
        if find_len_dijkstra(&bytes[..=mid], end) != 0 {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    bytes[low]
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_solution(input, 1024, 70, 70)
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_solution(input, 70, 70)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_solution(
            &advent_of_code::template::read_file("examples", DAY),
            12,
            6,
            6,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_solution(&advent_of_code::template::read_file("examples", DAY), 6, 6);
        assert_eq!(result, Some("6,1".to_string()));
    }
}
