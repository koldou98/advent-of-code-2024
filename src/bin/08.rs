use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let max_y = input.lines().filter(|l| !l.is_empty()).count();
    let max_x = input.lines().last().unwrap().len();
    let antennas: HashMap<char, Vec<(usize, usize)>> = parse_input(input);
    antennas.values().for_each(|v| {
        if v.len() == 1 {
            return;
        }
        for (i, antenna) in v.iter().enumerate() {
            for antenna_to_check in v.iter().skip(i + 1) {
                add_antinodes(&mut antinodes, antenna, antenna_to_check, max_y, max_x);
            }
        }
    });
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let max_y = input.lines().filter(|l| !l.is_empty()).count();
    let max_x = input.lines().last().unwrap().len();
    let antennas: HashMap<char, Vec<(usize, usize)>> = parse_input(input);
    antennas.values().for_each(|v| {
        if v.len() == 1 {
            return;
        }

        for (i, antenna) in v.iter().enumerate() {
            antinodes.insert((antenna.0 as isize, antenna.1 as isize));
            for antenna_to_check in v.iter().skip(i + 1) {
                add_antinodes_loop(&mut antinodes, antenna, antenna_to_check, max_y, max_x);
            }
        }
    });

    Some(antinodes.len())
}

fn parse_input(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, ch)| ch.is_ascii_alphabetic() || ch.is_ascii_digit())
            .for_each(|(x, ch)| {
                antennas
                    .entry(ch)
                    .and_modify(|antenna_set| {
                        antenna_set.push((y, x));
                    })
                    .or_insert(vec![(y, x)]);
            })
    });
    antennas
}

fn add_antinodes_loop(
    antinodes: &mut HashSet<(isize, isize)>,
    antenna: &(usize, usize),
    antenna_to_check: &(usize, usize),
    max_y: usize,
    max_x: usize,
) {
    let antenna = (antenna.0 as isize, antenna.1 as isize);
    let antenna_to_check = (antenna_to_check.0 as isize, antenna_to_check.1 as isize);

    let diff = (
        antenna.0.abs_diff(antenna_to_check.0),
        antenna.1.abs_diff(antenna_to_check.1),
    );
    let mut antenna_antinode = generate_antinode(&antenna, &antenna_to_check, &diff);
    let mut operation = (
        antenna_antinode.0 - antenna.0,
        antenna_antinode.1 - antenna.1,
    );
    while in_bounds(antenna_antinode, max_y, max_x) {
        antinodes.insert(antenna_antinode);
        antenna_antinode = (
            antenna_antinode.0 + operation.0,
            antenna_antinode.1 + operation.1,
        );
    }
    antenna_antinode = generate_antinode(&antenna_to_check, &antenna, &diff);
    operation = (-operation.0, -operation.1);
    while in_bounds(antenna_antinode, max_y, max_x) {
        antinodes.insert(antenna_antinode);
        antenna_antinode = (
            antenna_antinode.0 + operation.0,
            antenna_antinode.1 + operation.1,
        );
    }
}

fn add_antinodes(
    antinodes: &mut HashSet<(isize, isize)>,
    antenna: &(usize, usize),
    antenna_to_check: &(usize, usize),
    max_y: usize,
    max_x: usize,
) {
    let antenna = (antenna.0 as isize, antenna.1 as isize);
    let antenna_to_check = (antenna_to_check.0 as isize, antenna_to_check.1 as isize);

    let diff = (
        antenna.0.abs_diff(antenna_to_check.0),
        antenna.1.abs_diff(antenna_to_check.1),
    );

    let antenna_antinode = generate_antinode(&antenna, &antenna_to_check, &diff);
    let antenna_to_check_antinode = generate_antinode(&antenna_to_check, &antenna, &diff);
    if in_bounds(antenna_antinode, max_y, max_x) {
        antinodes.insert(antenna_antinode);
    }
    if in_bounds(antenna_to_check_antinode, max_y, max_x) {
        antinodes.insert(antenna_to_check_antinode);
    }
}

fn generate_antinode(
    antenna: &(isize, isize),
    other_antenna: &(isize, isize),
    diff: &(usize, usize),
) -> (isize, isize) {
    let y = match antenna.0 < other_antenna.0 {
        true => antenna.0 - diff.0 as isize,
        false => antenna.0 + diff.0 as isize,
    };

    let x = match antenna.1 < other_antenna.1 {
        true => antenna.1 - diff.1 as isize,
        false => antenna.1 + diff.1 as isize,
    };
    (y, x)
}

fn in_bounds(antinode: (isize, isize), max_y: usize, max_x: usize) -> bool {
    if antinode.0 < 0
        || antinode.0 >= max_y as isize
        || antinode.1 < 0
        || antinode.1 >= max_x as isize
    {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
