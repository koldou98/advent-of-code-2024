advent_of_code::solution!(14);

type RobotConfig = ((isize, isize), (isize, isize));

pub fn part_one_testing(input: &str) -> Option<u64> {
    const COL: usize = 11;
    const ROW: usize = 7;
    let input = parse_input(input);
    let seconds = 100;

    let c_delim = COL / 2;
    let r_delim = ROW / 2;

    let safety_factor = calculate_safety_factor(&input, seconds, c_delim, r_delim, COL, ROW);
    Some(safety_factor)
}

pub fn part_one(input: &str) -> Option<u64> {
    const COL: usize = 101;
    const ROW: usize = 103;
    let input = parse_input(input);
    let seconds = 100;
    let c_delim = COL / 2;
    let r_delim = ROW / 2;

    let safety_factor = calculate_safety_factor(&input, seconds, c_delim, r_delim, COL, ROW);
    Some(safety_factor)
}
pub fn part_two(input: &str) -> Option<u64> {
    const COL: usize = 101;
    const ROW: usize = 103;
    let input = parse_input(input);
    let c_delim = COL / 2;
    let r_delim = ROW / 2;
    let mut min_safety_factor = u64::MAX;
    let mut min_safety_factor_iteration = 0;
    for i in 0..=COL * ROW {
        let safety_factor = calculate_safety_factor(&input, i as isize, c_delim, r_delim, COL, ROW);
        if safety_factor < min_safety_factor {
            min_safety_factor = safety_factor;
            min_safety_factor_iteration = i;
        }
    }
    Some(min_safety_factor_iteration as u64)
}

fn calculate_safety_factor(
    input: &[RobotConfig],
    seconds: isize,
    c_delim: usize,
    r_delim: usize,
    col: usize,
    row: usize,
) -> u64 {
    let mut quadrants = [0; 4];
    input.iter().for_each(|(d, v)| {
        let final_total_x = (v.0) * seconds + d.0;
        let final_total_y = (v.1) * seconds + d.1;
        let rel_x = final_total_x.unsigned_abs() % col;
        let x = if final_total_x < 0 && rel_x != 0 {
            col - rel_x
        } else {
            rel_x
        };
        let rel_y = final_total_y.unsigned_abs() % row;
        let y = if final_total_y < 0 && rel_y != 0 {
            row - rel_y
        } else {
            rel_y
        };
        match (x, y) {
            (x, y) if x < c_delim && y < r_delim => quadrants[0] += 1,
            (x, y) if x < c_delim && y > r_delim => quadrants[1] += 1,
            (x, y) if x > c_delim && y < r_delim => quadrants[2] += 1,
            (x, y) if x > c_delim && y > r_delim => quadrants[3] += 1,
            _ => {}
        }
    });
    quadrants.iter().product()
}

fn parse_input(input: &str) -> Vec<RobotConfig> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            let p_separator = p.find(',').unwrap();
            let v_separator = v.find(',').unwrap();
            let x = &p[2..p_separator].parse::<isize>().unwrap();
            let y = &p[p_separator + 1..].parse::<isize>().unwrap();
            let vx = &v[2..v_separator].parse::<isize>().unwrap();
            let vy = &v[v_separator + 1..].parse::<isize>().unwrap();
            ((*x, *y), (*vx, *vy))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_testing(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
