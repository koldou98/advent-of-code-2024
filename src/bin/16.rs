use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // North, East, South, West

pub fn part_one(input: &str) -> Option<u64> {
    let mut start_location = (0, 0);
    let mut end_location = (0, 0);
    let map = parse_input(input, &mut start_location, &mut end_location);
    let result = calculate_cost_dijkstra(&map, start_location, end_location, 1);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut start_location = (0, 0);
    let mut end_location = (0, 0);
    let map = parse_input(input, &mut start_location, &mut end_location);
    let result = calculate_best_path_tiles_count(&map, start_location, end_location, 1);
    Some(result)
}

fn calculate_cost_dijkstra(map: &[Vec<char>], start: Point, end: Point, dir: usize) -> u64 {
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::from([Reverse((0, start.0, start.1, dir))]);
    while !heap.is_empty() {
        let (cost, r, c, dir) = heap.pop().unwrap().0;
        if r == end.0 && c == end.1 {
            return cost;
        }
        if visited.contains(&(r, c, dir)) {
            continue;
        }
        visited.insert((r, c, dir));
        let (dir_r, dir_c) = DIRECTIONS[dir];
        let (next_r, next_c) = (r as isize + dir_r, c as isize + dir_c);
        for (next_cost, next_r, next_c, next_dir) in [
            (cost + 1, next_r as usize, next_c as usize, dir),
            (cost + 1000, r, c, (dir + 1) % 4),
            (cost + 1000, r, c, (dir + 3) % 4),
        ] {
            if map[next_r][next_c] == '#' {
                continue;
            }
            heap.push(Reverse((next_cost, next_r, next_c, next_dir)));
        }
    }
    u64::MAX
}

fn calculate_best_path_tiles_count(map: &[Vec<char>], start: Point, end: Point, dir: usize) -> u64 {
    let mut visited: HashMap<(usize, usize, usize), u64> = HashMap::new();
    let mut heap = BinaryHeap::from([Reverse((0, start.0, start.1, dir))]);
    let mut best_cost = u64::MAX;
    let mut backtrack = HashMap::new();
    let mut end_cases = HashSet::new();
    while !heap.is_empty() {
        let (cost, r, c, dir) = heap.pop().unwrap().0;
        if cost > *visited.get(&(r, c, dir)).unwrap_or(&u64::MAX) {
            continue;
        }
        if r == end.0 && c == end.1 {
            if cost > best_cost {
                break;
            }
            best_cost = cost;
            end_cases.insert((r, c, dir));
        }
        let (dir_r, dir_c) = DIRECTIONS[dir];
        let (next_r, next_c) = (r as isize + dir_r, c as isize + dir_c);
        for (new_cost, new_r, new_c, new_dir) in [
            (cost + 1, next_r as usize, next_c as usize, dir),
            (cost + 1000, r, c, (dir + 1) % 4),
            (cost + 1000, r, c, (dir + 3) % 4),
        ] {
            if map[new_r][new_c] == '#' {
                continue;
            }
            let visited_lowest = *visited.get(&(new_r, new_c, new_dir)).unwrap_or(&u64::MAX);
            if new_cost > visited_lowest {
                continue;
            }
            if new_cost < visited_lowest {
                backtrack
                    .entry((new_r, new_c, new_dir))
                    .insert_entry(HashSet::new());
                visited
                    .entry((new_r, new_c, new_dir))
                    .insert_entry(new_cost);
            }
            backtrack.entry((new_r, new_c, new_dir)).and_modify(|e| {
                e.insert((r, c, dir));
            });
            heap.push(Reverse((new_cost, new_r, new_c, new_dir)));
        }
    }
    let mut deque: VecDeque<(usize, usize, usize)> = VecDeque::from_iter(end_cases.clone());
    let mut seen: HashSet<(usize, usize, usize)> = HashSet::from_iter(end_cases);
    while !deque.is_empty() {
        let el = deque.pop_front().unwrap();
        for last in backtrack.get(&el).unwrap_or(&HashSet::new()) {
            if !seen.contains(last) {
                seen.insert(*last);
                deque.push_back(*last);
            }
        }
    }
    seen.iter()
        .map(|(r, c, _)| (r, c))
        .collect::<HashSet<_>>()
        .len() as u64
}

type Point = (usize, usize);

fn parse_input(
    input: &str,
    start_location: &mut Point,
    end_location: &mut Point,
) -> Vec<Vec<char>> {
    input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| {
                    match ch {
                        'E' => *end_location = (r, c),
                        'S' => *start_location = (r, c),
                        _ => {}
                    }
                    ch
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use advent_of_code::template::read_file_part;

    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(7036));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(&read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }
}
