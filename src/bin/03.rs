use crate::Instruction::{Do, Dont};

advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
enum Next {
    Num1,
    NumComma,
    Num2,
    NumEnd,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Do,
    Dont,
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut split = input.split("mul(");
    split.next();
    let mut result = 0;
    for content in split {
        result += map_content(content);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let split = input.split("mul(");
    let mut result = 0;
    let mut instruction = Do;
    for content in split {
        if instruction == Do {
            result += map_content(content);
        }
        instruction = get_instruction(content, instruction);
    }
    Some(result)
}

fn get_instruction(content: &str, instruction: Instruction) -> Instruction {
    let mut new_instruction = instruction;
    let matches = content.rsplit_once("do");
    match matches {
        None => {}
        Some((_, part)) => {
            if part.starts_with("n't") {
                new_instruction = Dont
            } else {
                new_instruction = Do
            }
        }
    }
    new_instruction
}

fn map_content(content: &str) -> u32 {
    let mut result = 0;
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut first_check = false;
    let mut second_check = false;
    let mut next = Next::Num1;
    for char in content.chars() {
        match next {
            Next::Num1 => {
                if first_number.len() < 4 && char.is_ascii_digit() {
                    first_number.push(char);
                    next = Next::NumComma;
                } else {
                    break;
                }
            }
            Next::NumComma => {
                if first_number.len() < 4 && char.is_ascii_digit() {
                    first_number.push(char);
                } else if char == ',' {
                    first_check = true;
                    next = Next::Num2;
                } else {
                    break;
                }
            }
            Next::Num2 => {
                if second_number.len() < 4 && char.is_ascii_digit() {
                    second_number.push(char);
                    next = Next::NumEnd;
                } else {
                    break;
                }
            }
            Next::NumEnd => {
                if second_number.len() < 4 && char.is_ascii_digit() {
                    second_number.push(char);
                    next = Next::NumEnd;
                } else if char == ')' {
                    second_check = true;
                    break;
                } else {
                    break;
                }
            }
        }
    }
    if first_check && second_check {
        result = first_number.parse::<u32>().unwrap() * second_number.parse::<u32>().unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
