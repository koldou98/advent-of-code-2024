use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u64> {
    let connection_map = parse_input(input);

    let mut sets = HashSet::new();

    for x in connection_map.keys() {
        for y in connection_map[x].iter() {
            for z in connection_map[y].iter() {
                if x != z && connection_map[z].contains(x) {
                    let mut vec = vec![x, y, z];
                    vec.sort();
                    sets.insert(vec);
                }
            }
        }
    }
    let result = sets
        .iter()
        .filter(|set| set.iter().any(|a| a.starts_with("t")))
        .count() as u64;
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let connection_map = parse_input(input);
    let mut sets = HashSet::new();
    for computer in connection_map.keys() {
        search_connected_computers(
            &connection_map,
            &mut sets,
            computer,
            HashSet::from([computer]),
        );
    }
    let mut result = sets
        .iter()
        .max_by_key(|set| set.len())
        .unwrap()
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>();
    result.sort();
    Some(result.join(","))
}

fn search_connected_computers<'a>(
    connection_map: &'a HashMap<&'a str, HashSet<&'a str>>,
    sets: &mut HashSet<Vec<&&'a str>>,
    computer: &'a &str,
    connections: HashSet<&'a &str>,
) {
    let mut vec = connections.clone().into_iter().collect::<Vec<_>>();
    vec.sort();
    if sets.contains(&vec) {
        return;
    }

    sets.insert(vec);
    for connected in connection_map.get(computer).unwrap() {
        if connections.contains(&connected) {
            continue;
        }
        if !connections.iter().all(|con| {
            connection_map
                .get(con.to_string().as_str())
                .unwrap()
                .contains(connected)
        }) {
            continue;
        }
        let mut new_connections = connections.clone();
        new_connections.insert(connected);
        search_connected_computers(connection_map, sets, computer, new_connections);
    }
}

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        map.entry(a)
            .and_modify(|v| {
                v.insert(b);
            })
            .or_insert(HashSet::from([b]));
        map.entry(b)
            .and_modify(|v| {
                v.insert(a);
            })
            .or_insert_with(|| HashSet::from([a]));
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
