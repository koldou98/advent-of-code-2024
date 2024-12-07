advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let operations = map_content(input);
    let result: usize = operations
        .iter()
        .filter_map(|(result, numbers)| {
            if numbers.len() == 1 && result == numbers.first().unwrap() {
                return Some(*result);
            } else {
                let mut operation_result = vec![];
                for i in 1..numbers.len() {
                    let left_num = numbers[i - 1];
                    let right_num = numbers[i];
                    if operation_result.is_empty() {
                        operation_result.push(left_num * right_num);
                        operation_result.push(left_num + right_num);
                    } else {
                        for j in (0..operation_result.len()).rev() {
                            let mult_value = operation_result[j] * numbers[i];
                            if mult_value <= *result {
                                operation_result.push(mult_value);
                            }
                            let sum_value = operation_result[j] + numbers[i];
                            if sum_value > *result {
                                operation_result.remove(j);
                            } else {
                                operation_result[j] = sum_value;
                            }
                        }
                    }
                }
                if operation_result.iter().any(|element| element == result) {
                    return Some(*result);
                }
            }
            None
        })
        .sum();
    Some(result)
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

pub fn part_two(input: &str) -> Option<usize> {
    let operations = map_content(input);
    let result: usize = operations
        .iter()
        .filter_map(|(result, numbers)| {
            if numbers.len() == 1 && result == numbers.first().unwrap() {
                return Some(*result);
            } else {
                let mut operation_result = vec![];
                for i in 1..numbers.len() {
                    let left_num = numbers[i - 1];
                    let right_num = numbers[i];
                    if operation_result.is_empty() {
                        operation_result.push(left_num * right_num);
                        operation_result.push(left_num + right_num);
                        let string = left_num.to_string() + &right_num.to_string();
                        operation_result.push(string.parse().unwrap());
                    } else {
                        for j in (0..operation_result.len()).rev() {
                            let string_value =
                                operation_result[j].to_string() + &numbers[i].to_string();
                            let usize_string_value = string_value.parse::<usize>().unwrap();
                            if usize_string_value <= *result {
                                operation_result.push(usize_string_value);
                            }

                            let mult_value = operation_result[j] * numbers[i];
                            if mult_value <= *result {
                                operation_result.push(mult_value);
                            }
                            let sum_value = operation_result[j] + numbers[i];
                            if sum_value > *result {
                                operation_result.remove(j);
                            } else {
                                operation_result[j] = sum_value;
                            }
                        }
                    }
                }
                if operation_result.iter().any(|element| element == result) {
                    return Some(*result);
                }
            }
            None
        })
        .sum();
    Some(result)
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
