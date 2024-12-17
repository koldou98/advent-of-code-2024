use std::collections::{HashSet, VecDeque};
use std::str::Lines;

advent_of_code::solution!(17);

#[derive(Debug, Clone, Default)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<usize>,
    output: Vec<String>,
    instruction_pointer: usize,
}

impl Computer {
    pub(crate) fn restart(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.output.clear();
        self.instruction_pointer = 0;
        self.reg_a = reg_a;
        self.reg_b = reg_b;
        self.reg_c = reg_c;
    }
}

impl Computer {
    fn run_program(&mut self) {
        while self.instruction_pointer < self.program.len() {
            let op = self.program[self.instruction_pointer];
            let comb_inst_pnt = self.instruction_pointer + 1;
            let lit_op = self.program[comb_inst_pnt];
            let combo_op = self.get_combo_op(lit_op);
            match op {
                0 => self.reg_a /= 2usize.pow(combo_op as u32),
                1 => self.reg_b ^= combo_op,
                2 => self.reg_b = combo_op % 8,
                3 => {
                    if self.reg_a != 0 {
                        self.instruction_pointer = lit_op;
                        continue;
                    }
                }
                4 => self.reg_b ^= self.reg_c,
                5 => self.output.push((combo_op % 8).to_string()),
                6 => self.reg_b = self.reg_a / 2usize.pow(combo_op as u32),
                7 => self.reg_c = self.reg_a / 2usize.pow(combo_op as u32),
                _ => {}
            }
            self.instruction_pointer += 2;
        }
    }

    fn get_combo_op(&mut self, lit_op: usize) -> usize {
        match lit_op {
            0..=3 => lit_op,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => lit_op,
        }
    }

    fn print_output(&self) -> String {
        self.output.join(",")
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();

    let (reg_a, reg_b, reg_c, program) = parse_input(&mut lines);
    let mut computer = Computer {
        reg_a,
        reg_b,
        reg_c,
        program,
        ..Default::default()
    };
    computer.run_program();
    Some(computer.print_output())
}

fn parse_input(lines: &mut Lines) -> (usize, usize, usize, Vec<usize>) {
    let reg_a = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let reg_b = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let reg_c = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let program: Vec<usize> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect();
    (reg_a, reg_b, reg_c, program)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();

    let (reg_a, reg_b, reg_c, program) = parse_input(&mut lines);
    let mut computer = Computer {
        reg_a,
        reg_b,
        reg_c,
        program,
        ..Default::default()
    };
    let candidates = get_quine_candidates(reg_b, reg_c, &mut computer);
    candidates.into_iter().min()
}

fn get_quine_candidates(reg_b: usize, reg_c: usize, computer: &mut Computer) -> HashSet<usize> {
    let program = computer.program.clone();
    let mut to_check = VecDeque::from([(computer.program.len() - 1, 0)]);
    let mut candidates = HashSet::new();
    while !to_check.is_empty() {
        let (pos, val) = to_check.pop_front().unwrap();
        let program_to_check = &program[pos..];
        for next_a_reg in val * 8..val * 8 + 8 {
            computer.restart(next_a_reg, reg_b, reg_c);
            computer.run_program();
            let vec = computer
                .output
                .iter()
                .map(|out| out.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if program_to_check == vec {
                if let Some(next_pos) = pos.checked_sub(1) {
                    to_check.push_back((next_pos, next_a_reg));
                };
                if pos == 0 {
                    candidates.insert(next_a_reg);
                }
            }
        }
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file_part;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(String::from("0,1,2")));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(String::from("4,2,5,6,7,7,7,7,3,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(117440));
    }
}
