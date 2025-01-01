use std::collections::HashMap;
use std::string::String;

advent_of_code::solution!(24);

fn calculate_value(
    memory: &mut HashMap<String, u8>,
    connections: &mut HashMap<String, (&str, String, String)>,
    wire: String,
) -> u8 {
    if memory.contains_key(&wire) {
        return memory[&wire];
    }
    let (op, input_a, input_b) = connections.get(&wire).unwrap();
    let operation_result = get_operation_result(
        op,
        calculate_value(memory, &mut connections.clone(), input_a.to_string()),
        calculate_value(memory, &mut connections.clone(), input_b.to_string()),
    );
    memory.insert(wire, operation_result);
    operation_result
}

fn get_operation_result(operation: &str, input_a: u8, input_b: u8) -> u8 {
    match operation {
        "AND" => input_a & input_b,
        "OR" => input_a | input_b,
        "XOR" => input_a ^ input_b,
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (inputs, connections) = input.split_once("\n\n").unwrap();
    let mut memory: HashMap<String, u8> = HashMap::new();
    inputs.lines().for_each(|input| {
        let (wire, value) = input.split_once(": ").unwrap();
        memory.insert(wire.to_string(), value.parse().unwrap());
    });

    let mut program = parse_connections(connections);
    let mut result = vec![];
    let mut i = 0;
    loop {
        let key = format!("z{:02}", i);
        if !program.contains_key(key.as_str()) {
            break;
        }
        result.push(calculate_value(&mut memory, &mut program, key));
        i += 1;
    }
    result.reverse();
    let mut result_string = String::new();
    for val in result {
        result_string.push_str(&val.to_string());
    }
    Some(u64::from_str_radix(&result_string, 2).unwrap())
}

fn parse_connections(connections: &str) -> HashMap<String, (&str, String, String)> {
    let mut program = HashMap::new();

    connections.lines().for_each(|connection| {
        let mut split = connection.split(' ');
        let input_a = split.next().unwrap();
        let gate = split.next().unwrap();
        let input_b = split.next().unwrap();
        let output = split.last().unwrap();
        program.insert(
            output.to_string(),
            (gate, input_a.to_string(), input_b.to_string()),
        );
    });
    program
}

fn find_next(program: &HashMap<String, (&str, String, String)>) -> usize {
    let mut i = 0;
    loop {
        if !verify_z(program, &format_wire('z', i), i) {
            break;
        }
        i += 1
    }
    i
}
pub fn part_two(input: &str) -> Option<String> {
    let (_, connections) = input.split_once("\n\n").unwrap();
    let mut program = parse_connections(connections);
    let mut swaps = vec![];
    let keys: Vec<String> = program.keys().cloned().collect();
    let mut continue_check = true;
    let mut start = 0;
    for _ in 0..4 {
        let baseline = find_next(&program);
        for i in start..keys.len() {
            for j in 0..keys.len() {
                if i == j {
                    continue;
                }
                let x = &keys[i];
                let y = &keys[j];

                let tmp_x = program.get(x).cloned().unwrap();
                let tmp_y = program.get(y).cloned().unwrap();

                program.insert(x.clone(), tmp_y.clone());
                program.insert(y.clone(), tmp_x.clone());
                let next = find_next(&program);
                if next > baseline {
                    swaps.push(x.clone());
                    swaps.push(y.clone());
                    continue_check = false;
                    start = next;
                    break;
                }
                let tmp_x = program.get(x).cloned().unwrap();
                let tmp_y = program.get(y).cloned().unwrap();
                program.insert(x.clone(), tmp_y.clone());
                program.insert(y.clone(), tmp_x.clone());
            }
            if !continue_check {
                continue_check = true;
                break;
            }
        }
    }
    swaps.sort();
    Some(swaps.join(","))
}

fn verify_z(program: &HashMap<String, (&str, String, String)>, wire: &str, num: usize) -> bool {
    if !program.contains_key(wire) {
        return false;
    }
    let (op, x, y) = &program[wire];
    if *op != "XOR" {
        return false;
    }
    if num == 0 {
        let mut vec = vec![x, y];
        vec.sort();
        return vec == vec!["x00", "y00"];
    }
    (verify_intermediate_xor(program, x, num) && verify_carry_bit(program, y, num))
        || (verify_intermediate_xor(program, y, num) && verify_carry_bit(program, x, num))
}

fn verify_carry_bit(
    program: &HashMap<String, (&str, String, String)>,
    wire: &str,
    number: usize,
) -> bool {
    let (op, input_a, input_b) = program.get(wire).unwrap();
    if number == 1 {
        if *op != "AND" {
            return false;
        }
        let mut out = vec![input_a, input_b];
        out.sort();
        return out == vec!["x00", "y00"];
    }
    if *op != "OR" {
        return false;
    }
    verify_direct_carry(program, input_a, number - 1)
        && verify_recursive_carry(program, input_b, number - 1)
        || verify_direct_carry(program, input_b, number - 1)
            && verify_recursive_carry(program, input_a, number - 1)
}

fn verify_recursive_carry(
    program: &HashMap<String, (&str, String, String)>,
    wire: &String,
    number: usize,
) -> bool {
    let (op, input_a, input_b) = program.get(wire).unwrap();
    if *op != "AND" {
        return false;
    }
    verify_intermediate_xor(program, input_a, number) && verify_carry_bit(program, input_b, number)
        || verify_intermediate_xor(program, input_b, number)
            && verify_carry_bit(program, input_a, number)
}

fn verify_direct_carry(
    program: &HashMap<String, (&str, String, String)>,
    wire: &String,
    number: usize,
) -> bool {
    let (op, input_a, input_b) = program.get(wire).unwrap();
    if *op != "AND" {
        return false;
    }
    let mut out = vec![input_a, input_b];
    out.sort();
    out == vec![&format_wire('x', number), &format_wire('y', number)]
}

fn verify_intermediate_xor(
    program: &HashMap<String, (&str, String, String)>,
    wire: &str,
    number: usize,
) -> bool {
    if !program.contains_key(wire) {
        return false;
    }
    let (op, x, y) = &program[wire];
    if *op != "XOR" {
        return false;
    }
    let mut v1 = vec![x.clone(), y.clone()];
    v1.sort();
    let mut v2 = vec![format_wire('x', number), format_wire('y', number)];
    v2.sort();
    v1 == v2
}

fn format_wire(name: char, number: usize) -> String {
    format!("{name}{number:02}")
}

#[allow(dead_code)]
fn debug_print(
    program: &HashMap<String, (&str, String, String)>,
    wire: String,
    depth: usize,
) -> String {
    if wire.starts_with(['x', 'y']) {
        return format!("{:depth$}{wire}", "  ");
    }
    let (op, input_a, input_b) = program.get(&wire).unwrap();
    format!(
        "{:<depth$}{op} ({wire})\n{}\n{}",
        "  ",
        debug_print(program, input_a.clone(), depth + 1),
        debug_print(program, input_b.clone(), depth + 1)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::read_file_part;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file_part("examples", DAY, 3));
        assert_eq!(result, Some("aaa,aoc,bbb,ccc,eee,ooo,z24,z99".to_string()));
    }
}
