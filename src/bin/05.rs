use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut printing_orders: Vec<Vec<u32>> = Vec::new();
    let mut future_pages: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut _past_pages: HashMap<u32, HashSet<u32>> = HashMap::new();
    parse_input(
        input,
        &mut printing_orders,
        &mut future_pages,
        &mut _past_pages,
    );

    let result = printing_orders
        .iter()
        .filter_map(|printing_order| {
            if has_valid_printing_order(&future_pages, &printing_order) {
                return Some(printing_order[printing_order.len() / 2]);
            }
            None::<u32>
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut printing_orders: Vec<Vec<u32>> = Vec::new();
    let mut future_pages: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut past_pages: HashMap<u32, HashSet<u32>> = HashMap::new();
    parse_input(
        input,
        &mut printing_orders,
        &mut future_pages,
        &mut past_pages,
    );
    let result = printing_orders
        .iter()
        .filter_map(|printing_order| {
            if !has_valid_printing_order(&future_pages, &printing_order) {
                let new_print_order = topology_sort(printing_order, &past_pages, &future_pages);
                if !new_print_order.is_empty() {
                    return Some(new_print_order[new_print_order.len() / 2]);
                }
            }
            None::<u32>
        })
        .sum();
    Some(result)
}

fn topology_sort(
    printing_order: &[u32],
    past_pages: &HashMap<u32, HashSet<u32>>,
    future_pages: &HashMap<u32, HashSet<u32>>,
) -> Vec<u32> {
    let mut new_print_order: Vec<u32> = Vec::new();
    // Nodes without edges
    let mut queue = VecDeque::new();
    let mut graph = HashMap::new();
    for page in printing_order.iter() {
        if let Some(future_page_rule_set) = future_pages.get(page) {
            let future_element_count = future_page_rule_set
                .iter()
                .filter(|page| printing_order.contains(page))
                .count();
            graph.insert(page, future_element_count);
            if future_element_count == 0 {
                queue.push_back(*page);
            }
        } else {
            queue.push_back(*page);
            graph.insert(page, 0);
        }
    }
    while !queue.is_empty() {
        let page = queue.pop_front().unwrap();
        new_print_order.push(page);

        if let Some(past_page_rule_set) = past_pages.get(&page) {
            for past_page in past_page_rule_set.iter() {
                if let Some(node_edges) = graph.get_mut(past_page) {
                    *node_edges -= 1;
                    if *node_edges == 0 {
                        queue.push_back(*past_page);
                    }
                }
            }
        }
    }
    if new_print_order.len() != printing_order.len() {
        panic!("Graph has at least 1 cycle");
    }
    new_print_order
}

fn has_valid_printing_order(
    future_ordering_rules: &HashMap<u32, HashSet<u32>>,
    printing_order: &&Vec<u32>,
) -> bool {
    let mut valid = true;
    for (i, page) in printing_order.iter().enumerate() {
        let mut page_set = &HashSet::new();
        if let Some(set) = future_ordering_rules.get(page) {
            page_set = set
        }
        if i < printing_order.len() - 1 {
            let pages_to_check = &printing_order[i + 1..];
            if page_set.iter().any(|page| pages_to_check.contains(page)) {
                valid = false;
                break;
            };
        }
    }
    valid
}

fn parse_input(
    input: &str,
    printing_orders: &mut Vec<Vec<u32>>,
    future_pages: &mut HashMap<u32, HashSet<u32>>,
    past_pages: &mut HashMap<u32, HashSet<u32>>,
) {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if let Some(ordering_rule) = line.split_once('|') {
                future_pages
                    .entry(ordering_rule.1.parse().unwrap())
                    .and_modify(|set| {
                        set.insert(ordering_rule.0.parse().unwrap());
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert(ordering_rule.0.parse().unwrap());
                        set
                    });
                past_pages
                    .entry(ordering_rule.0.parse().unwrap())
                    .and_modify(|set| {
                        set.insert(ordering_rule.1.parse().unwrap());
                    })
                    .or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert(ordering_rule.1.parse().unwrap());
                        set
                    });
            } else {
                printing_orders.push(line.split(',').map(|x| x.parse::<u32>().unwrap()).collect());
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
