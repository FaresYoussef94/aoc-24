use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let input_vec: Vec<&str> = input.split("\n\n").collect();
    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in input_vec.first().unwrap().lines() {
        let parts: Vec<&str> = rule.split('|').collect();
        if parts.len() == 2 {
            if let (Ok(key), Ok(value)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                rules_map.entry(key).or_default().push(value);
            }
        }
    }

    let mut result: u32 = 0;

    for updates in input_vec.last().unwrap().lines() {
        let updates_vec: Vec<u32> = updates
            .split(",")
            .map(|update| update.parse::<u32>().ok().unwrap())
            .collect();

        if is_safe(&rules_map, &updates_vec) {
            result += updates_vec.get(updates_vec.len() / 2).unwrap();
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_vec: Vec<&str> = input.split("\n\n").collect();
    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in input_vec.first().unwrap().lines() {
        let parts: Vec<&str> = rule.split('|').collect();
        if parts.len() == 2 {
            if let (Ok(key), Ok(value)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                rules_map.entry(key).or_default().push(value);
            }
        }
    }

    let mut incorrect_updates: Vec<Vec<u32>> = Vec::new();

    for updates in input_vec.last().unwrap().lines() {
        let updates_vec: Vec<u32> = updates
            .split(",")
            .map(|update| update.parse::<u32>().ok().unwrap())
            .collect();

        if !is_safe(&rules_map, &updates_vec) {
            incorrect_updates.push(updates_vec);
        }
    }

    for incor_update in incorrect_updates.iter_mut() {
        incor_update.sort_by(|a, b| {
            if rules_map.contains_key(a) && rules_map.get(a).unwrap().contains(b) {
                return Ordering::Less;
            } else if rules_map.contains_key(b) && rules_map.get(b).unwrap().contains(a) {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        });
    }

    let mut result: u32 = 0;

    for updates in incorrect_updates {
        result += updates.get(updates.len() / 2).unwrap();
    }
    Some(result)
}

fn is_safe(rules_map: &HashMap<u32, Vec<u32>>, updates_vec: &Vec<u32>) -> bool {
    for i in 0..updates_vec.len() {
        let curr_num = updates_vec.get(i).unwrap();
        if rules_map.contains_key(curr_num) {
            let latter_numbers = rules_map.get(curr_num).unwrap();
            for j in 0..i {
                let latter_num = updates_vec.get(j).unwrap();
                if latter_numbers.contains(latter_num) {
                    return false;
                }
            }
        }
    }
    true
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
