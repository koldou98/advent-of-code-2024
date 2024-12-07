advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let concatenation = false;
    let operations = map_content(input);
    let result: usize = get_calibration_result(operations, concatenation);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let concatenation = true;
    let operations = map_content(input);
    let result: usize = get_calibration_result(operations, concatenation);
    Some(result)
}

fn get_calibration_result(operations: Vec<(usize, Vec<usize>)>, concatenation: bool) -> usize {
    operations
        .iter()
        .filter_map(|(result, numbers)| {
            if numbers.len() == 1 && result == numbers.first().unwrap() {
                return Some(*result);
            } else {
                let mut operation_result = vec![];
                for i in 1..numbers.len() {
                    if operation_result.is_empty() {
                        apply_first_operation(i, numbers, &mut operation_result, concatenation);
                    } else {
                        apply_operations(i, numbers, &mut operation_result, concatenation, result);
                    }
                }
                if operation_result.iter().any(|element| element == result) {
                    return Some(*result);
                }
            }
            None
        })
        .sum()
}

fn apply_first_operation(
    index: usize,
    numbers: &[usize],
    operation_result: &mut Vec<usize>,
    concatenation: bool,
) {
    let left_num = numbers[index - 1];
    let right_num = numbers[index];
    operation_result.push(left_num * right_num);
    operation_result.push(left_num + right_num);
    if concatenation {
        let string = left_num.to_string() + &right_num.to_string();
        operation_result.push(string.parse().unwrap());
    }
}

fn apply_operations(
    index: usize,
    numbers: &[usize],
    operation_result: &mut Vec<usize>,
    concatenation: bool,
    result: &usize,
) {
    for j in (0..operation_result.len()).rev() {
        if concatenation {
            let string_value = operation_result[j].to_string() + &numbers[index].to_string();
            let usize_string_value = string_value.parse::<usize>().unwrap();
            if usize_string_value <= *result {
                operation_result.push(usize_string_value);
            }
        }
        let mult_value = operation_result[j] * numbers[index];
        if mult_value <= *result {
            operation_result.push(mult_value);
        }
        let sum_value = operation_result[j] + numbers[index];
        if sum_value > *result {
            operation_result.remove(j);
        } else {
            operation_result[j] = sum_value;
        }
    }
}

fn map_content(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (result, numbers) = line.split_once(':').unwrap();
            let result = result.parse::<usize>().unwrap();
            let numbers = numbers
                .trim()
                .split(' ')
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (result, numbers)
        })
        .collect::<Vec<(usize, Vec<usize>)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
