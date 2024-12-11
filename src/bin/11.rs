use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let initial_stones: Vec<u64> = input
        .split_whitespace()
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    Some(count_stones(initial_stones, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_stones: Vec<u64> = input
        .split_whitespace()
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let mut result: u64 = 0;
    let mut memo: HashMap<String, u64> = HashMap::new();

    for stone in initial_stones.iter() {
        result += count_with_map(*stone, 0, 75, &mut memo);
    }

    Some(result)
}

fn count_stones(initial_stones: Vec<u64>, iterations: usize) -> u32 {
    let mut prev_stones = initial_stones.clone();
    let mut curr_stones: Vec<u64> = Vec::new();

    for _i in 0..iterations {
        for stone in prev_stones.iter() {
            let stone_as_str = stone.to_string();
            let stone_count = stone_as_str.chars().count();
            if stone == &0 {
                curr_stones.push(1);
            } else if stone_count % 2 == 0 {
                curr_stones.push(stone_as_str[0..stone_count / 2].parse::<u64>().unwrap());
                curr_stones.push(stone_as_str[stone_count / 2..].parse::<u64>().unwrap());
            } else {
                curr_stones.push(stone * 2024);
            }
        }
        prev_stones = curr_stones.clone();
        curr_stones.clear();
    }

    u32::try_from(prev_stones.len()).unwrap()
}

fn count_with_map(stone: u64, i: usize, max_i: usize, memo: &mut HashMap<String, u64>) -> u64 {
    if i == max_i {
        return 1;
    }

    let key_str = stone.to_string() + "-" + &(max_i - i).to_string();

    if memo.contains_key(&key_str) {
        return *memo.get(&key_str).unwrap();
    }

    let mut num: u64 = 0;
    let stone_as_str = stone.to_string();
    let stone_count = stone_as_str.chars().count();

    if stone == 0 {
        num += count_with_map(1, i + 1, max_i, memo);
    } else if stone_count % 2 == 0 {
        num += count_with_map(
            stone_as_str[0..stone_count / 2].parse::<u64>().unwrap(),
            i + 1,
            max_i,
            memo,
        );
        num += count_with_map(
            stone_as_str[stone_count / 2..].parse::<u64>().unwrap(),
            i + 1,
            max_i,
            memo,
        );
    } else {
        num += count_with_map(stone * 2024, i + 1, max_i, memo);
    }

    memo.insert(key_str, num);

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
