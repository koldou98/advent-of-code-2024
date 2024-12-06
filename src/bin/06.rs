use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut visited_positions = HashSet::new();
    let mut position = (0, 0);

    let map: Vec<Vec<char>> = generate_map(input, &mut visited_positions, &mut position);
    simulate_guard_route(&mut visited_positions, &position, &map);
    Some(visited_positions.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut visited_positions = HashSet::new();
    let mut initial_position = (0, 0);

    let map: Vec<Vec<char>> = generate_map(input, &mut visited_positions, &mut initial_position);
    simulate_guard_route(&mut visited_positions, &initial_position, &map);
    let loops = simulate_guard_route_with_additional_obstacles(
        &mut visited_positions,
        &initial_position,
        &map,
    );
    Some(loops)
}

fn simulate_guard_route_with_additional_obstacles(
    possible_obstacles: &mut HashSet<(isize, isize)>,
    initial_position: &(isize, isize),
    map: &[Vec<char>],
) -> usize {
    let mut loops = 0;
    for possible_obstacle in possible_obstacles.iter() {
        let mut visited_positions = HashSet::new();
        let mut position = *initial_position;
        let mut direction = (-1, 0);
        loop {
            let new_position = (position.0 + direction.0, position.1 + direction.1);
            if new_position.0 < 0
                || new_position.0 >= map.len() as isize
                || new_position.1 < 0
                || new_position.1 >= map[0].len() as isize
            {
                break;
            }
            if map[new_position.0 as usize][new_position.1 as usize] == '#'
                || new_position == *possible_obstacle
            {
                change_direction(&mut direction);
            } else if visited_positions.contains(&(new_position, direction)) {
                loops += 1;
                break;
            } else {
                visited_positions.insert((new_position, direction));
                position = new_position;
            }
        }
    }
    loops
}

fn change_direction(direction: &mut (isize, isize)) {
    match direction {
        (-1, 0) => *direction = (0, 1),
        (0, 1) => *direction = (1, 0),
        (1, 0) => *direction = (0, -1),
        (0, -1) => *direction = (-1, 0),
        _ => unreachable!(),
    }
}

fn generate_map(
    input: &str,
    visited_positions: &mut HashSet<(isize, isize)>,
    position: &mut (isize, isize),
) -> Vec<Vec<char>> {
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(y, line)| {
            let starting_position_x = line.chars().position(|ch| ch == '^');
            if let Some(x) = starting_position_x {
                *position = (y as isize, x as isize);
                visited_positions.insert((y as isize, x as isize));
            }
            line.chars().collect()
        })
        .collect()
}

fn simulate_guard_route(
    visited_positions: &mut HashSet<(isize, isize)>,
    initial_position: &(isize, isize),
    map: &[Vec<char>],
) {
    let mut position = *initial_position;
    let mut direction = (-1, 0);
    loop {
        let new_position = (position.0 + direction.0, position.1 + direction.1);
        if is_position_out_of_bounds(map, new_position) {
            break;
        }
        if map[new_position.0 as usize][new_position.1 as usize] == '#' {
            match direction {
                (-1, 0) => direction = (0, 1),
                (0, 1) => direction = (1, 0),
                (1, 0) => direction = (0, -1),
                (0, -1) => direction = (-1, 0),
                _ => unreachable!(),
            }
        } else {
            visited_positions.insert(new_position);
            position = new_position;
        }
    }
}

fn is_position_out_of_bounds(map: &[Vec<char>], new_position: (isize, isize)) -> bool {
    new_position.0 < 0
        || new_position.0 >= map.len() as isize
        || new_position.1 < 0
        || new_position.1 >= map[0].len() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
