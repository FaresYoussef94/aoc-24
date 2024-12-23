use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|row| {
        let parts: Vec<_> = row.split("-").collect();
        network
            .entry(parts[0].to_string())
            .and_modify(|val| val.push(parts[1].to_string()))
            .or_insert(vec![parts[1].to_string()]);
        network
            .entry(parts[1].to_string())
            .and_modify(|val| val.push(parts[0].to_string()))
            .or_insert(vec![parts[0].to_string()]);
    });

    let mut sets: Vec<Vec<String>> = vec![];

    for key in network.keys() {
        let mut curr_set = vec![];
        let mut visited: HashSet<String> = HashSet::new();
        get_sets(&network, 3, key, &mut sets, &mut curr_set, &mut visited);
    }

    Some(
        sets.iter()
            .filter(|set| set.len() == 3)
            .filter(|set| {
                set[0].starts_with("t") || set[1].starts_with("t") || set[2].starts_with("t")
            })
            .count()
            / 6,
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|row| {
        let parts: Vec<_> = row.split("-").collect();
        network
            .entry(parts[0].to_string())
            .and_modify(|val| val.push(parts[1].to_string()))
            .or_insert(vec![parts[1].to_string()]);
        network
            .entry(parts[1].to_string())
            .and_modify(|val| val.push(parts[0].to_string()))
            .or_insert(vec![parts[0].to_string()]);
    });

    let cliques = bron_kerbosch(&network);
    let mut max: Vec<String> = cliques
        .into_iter()
        .max_by_key(|r| r.len())
        .unwrap()
        .into_iter()
        .collect();
    max.sort();

    Some(max.join(","))
}

fn get_sets(
    network: &HashMap<String, Vec<String>>,
    i: usize,
    curr_node: &String,
    gset: &mut Vec<Vec<String>>,
    curr_set: &mut Vec<String>,
    visited: &mut HashSet<String>,
) {
    if !curr_set.is_empty() && *curr_node == curr_set[0] {
        gset.push(curr_set.to_vec());
    }
    if i == 0 {
        return;
    }

    if visited.contains(curr_node) {
        return;
    }

    curr_set.push(curr_node.to_string());
    visited.insert(curr_node.to_string());

    for neighbours in network.get(curr_node).unwrap().iter() {
        get_sets(
            network,
            i - 1,
            neighbours,
            gset,
            &mut curr_set.clone(),
            &mut visited.clone(),
        );
    }
}

fn bron_kerbosch(network: &HashMap<String, Vec<String>>) -> Vec<HashSet<String>> {
    let mut result: Vec<HashSet<String>> = Vec::new();
    let vertices: HashSet<String> = network.keys().cloned().collect();

    fn bk_recursive(
        r: &mut HashSet<String>,
        p: &mut HashSet<String>,
        x: &mut HashSet<String>,
        graph: &HashMap<String, Vec<String>>,
        result: &mut Vec<HashSet<String>>,
    ) {
        if p.is_empty() && x.is_empty() {
            result.push(r.clone());
            return;
        }

        for v in p.clone().iter().cloned() {
            let mut r_new = r.clone();
            r_new.insert(v.clone());

            let mut p_new: HashSet<String> = p
                .intersection(
                    &graph
                        .get(&v)
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .collect(),
                )
                .cloned()
                .collect();

            let mut x_new: HashSet<String> = x
                .intersection(
                    &graph
                        .get(&v)
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .collect(),
                )
                .cloned()
                .collect();

            bk_recursive(&mut r_new, &mut p_new, &mut x_new, graph, result);

            p.remove(&v);
            x.insert(v);
        }
    }

    let mut p = vertices.clone();
    let mut x = HashSet::new();
    let mut r = HashSet::new();

    bk_recursive(&mut r, &mut p, &mut x, network, &mut result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
