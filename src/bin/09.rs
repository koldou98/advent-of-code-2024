use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut list = parse_input(input);
    let result = list
        .clone()
        .iter()
        .enumerate()
        .map(|(i, num)| {
            let mut total = 0;
            while *list.back().unwrap() == -1 {
                list.pop_back();
            }
            if *num == -1 {
                list.swap_remove_back(i);
            }
            if let Some(element) = list.get(i) {
                total = (element * i as isize) as usize;
            }
            total
        })
        .sum::<usize>();

    Some(result)
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut list = parse_input(input);
    let mut previous_file = -1;
    let mut file_size = 0;
    let mut checked_files = HashSet::new();
    for i in (0..list.len()).rev() {
        let file = *list.get(i).unwrap();
        if previous_file != file {
            if file_size > 0 && previous_file != -1 && !checked_files.contains(&previous_file) {
                checked_files.insert(previous_file);
                let mut space_count = 0;
                for j in 0..=i {
                    if *list.get(j).unwrap() == -1 {
                        space_count += 1;
                        if space_count >= file_size {
                            for swap in 0..file_size {
                                list.swap(j - swap, i + 1 + swap)
                            }
                            break;
                        }
                    } else {
                        space_count = 0;
                    }
                }
            }
            file_size = if file == -1 { 0 } else { 1 };
            previous_file = file;
        } else if file == -1 {
            previous_file = -1;
            file_size = 0
        } else if previous_file == file {
            file_size += 1;
        }
    }
    let result: usize = list
        .iter()
        .enumerate()
        .filter(|(_, file)| **file != -1)
        .map(|(i, file)| (file * i as isize) as usize)
        .sum();
    Some(result)
}
fn parse_input(input: &str) -> VecDeque<isize> {
    let mut list = VecDeque::new();
    let mut count = 0;
    let mut space = false;

    input.chars().for_each(|ch| {
        if let Some(num) = ch.to_digit(10) {
            let num = num as isize;
            match space {
                true => {
                    for _ in 0..num {
                        list.push_back(-1);
                    }
                    space = !space;
                }
                false => {
                    for _ in 0..num {
                        list.push_back(count);
                    }
                    count += 1;
                    space = !space;
                }
            }
        }
    });
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

// 6301361979006
// 6301302557053
