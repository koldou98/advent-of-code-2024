advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<isize> {
    let mut result = 0;
    let machines = input.split("\n\n");
    for machine in machines {
        let mut machine_config = machine.lines();
        let (ax, ay): (isize, isize) = parse_button(machine_config.next().unwrap());
        let (bx, by): (isize, isize) = parse_button(machine_config.next().unwrap());
        let (px, py): (isize, isize) = parse_price(machine_config.next().unwrap());
        result += solve_part_1(ax, ay, bx, by, px, py);
    }
    Some(result)
}

fn solve_part_1(ax: isize, ay: isize, bx: isize, by: isize, px: isize, py: isize) -> isize {
    let a_price = 3;
    let b_price = 1;
    let a_presses = (py * bx - px * by) / (ay * bx - ax * by);
    let b_presses = (px - a_presses * ax) / bx;
    // a_presses * ax + b_presses * bx = px
    // p_presses * ay + b_presses * by = py
    // Solve equation system
    if a_presses * ax + b_presses * bx != px
        || a_presses * ay + b_presses * by != py
        || a_presses > 100
        || b_presses > 100
    {
        return 0;
    }
    a_price * a_presses + b_price * b_presses
}

fn solve_part2(ax: isize, ay: isize, bx: isize, by: isize, px: isize, py: isize) -> isize {
    let a_price = 3;
    let b_price = 1;
    let a_presses = (py * bx - px * by) / (ay * bx - ax * by);
    let b_presses = (px - a_presses * ax) / bx;
    if a_presses * ax + b_presses * bx != px || a_presses * ay + b_presses * by != py {
        return 0;
    }
    a_price * a_presses + b_price * b_presses
}

fn parse_price(machine_config: &str) -> (isize, isize) {
    let x = machine_config
        .split_once("X=")
        .unwrap()
        .1
        .split_once(',')
        .unwrap()
        .0
        .parse::<isize>()
        .unwrap();

    let y = machine_config
        .rsplit_once('=')
        .unwrap()
        .1
        .parse::<isize>()
        .unwrap();
    (x, y)
}

fn parse_button(machine_config: &str) -> (isize, isize) {
    let x = machine_config
        .split_once("X+")
        .unwrap()
        .1
        .split_once(',')
        .unwrap()
        .0
        .parse::<isize>()
        .unwrap();

    let y = machine_config
        .rsplit_once('+')
        .unwrap()
        .1
        .parse::<isize>()
        .unwrap();
    (x, y)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut result = 0;
    let machines = input.split("\n\n");
    let prize_extra = 10000000000000;
    for machine in machines {
        let mut machine_config = machine.lines();
        let (ax, ay): (isize, isize) = parse_button(machine_config.next().unwrap());
        let (bx, by): (isize, isize) = parse_button(machine_config.next().unwrap());
        let (mut px, mut py): (isize, isize) = parse_price(machine_config.next().unwrap());
        px += prize_extra;
        py += prize_extra;
        result += solve_part2(ax, ay, bx, by, px, py);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
