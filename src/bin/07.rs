use core::str;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        // .map(|line| String::from(line))
        .collect();

    let mut gresult: u64 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(":").filter(|part| !part.is_empty()).collect();
        let result: u64 = parts.first().unwrap().parse().unwrap();
        let nums: Vec<u64> = parts
            .last()
            .unwrap()
            .split_whitespace()
            .filter(|el| !el.is_empty())
            .map(|el| el.parse().unwrap())
            .collect();

        if is_true(&nums, result, nums.len() - 1) {
            gresult += result;
        }
    }

    Some(gresult)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        // .map(|line| String::from(line))
        .collect();

    let mut gresult: u64 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(":").filter(|part| !part.is_empty()).collect();
        let result: u64 = parts.first().unwrap().parse().unwrap();
        let nums: Vec<u64> = parts
            .last()
            .unwrap()
            .split_whitespace()
            .filter(|el| !el.is_empty())
            .map(|el| el.parse().unwrap())
            .collect();

        if is_true_with_concat(&nums, result, nums.len() - 1) {
            gresult += result;
        }
    }

    Some(gresult)
}

fn is_true(input: &Vec<u64>, result: u64, i: usize) -> bool {
    if i == 0 && result != *input.first().unwrap() {
        return false;
    }

    if i == 0 && result == *input.first().unwrap() {
        return true;
    }

    let curr_num = input.get(i).unwrap();

    if *curr_num > result {
        return false;
    }

    let sub_result = result - input.get(i).unwrap();
    let div_result = result / input.get(i).unwrap();

    let should_div_path = div_result * input.get(i).unwrap() == result;

    is_true(input, sub_result, i - 1) || (should_div_path && is_true(input, div_result, i - 1))
}

fn is_true_with_concat(input: &Vec<u64>, result: u64, i: usize) -> bool {
    if i == 0 && result != *input.first().unwrap() {
        return false;
    }

    if i == 0 && result == *input.first().unwrap() {
        return true;
    }

    let curr_num = input.get(i).unwrap();

    if *curr_num > result {
        return false;
    }

    let curr_num = input.get(i).unwrap();

    let sub_result = result - curr_num;
    let div_result = result / curr_num;

    let should_div_path = div_result * curr_num == result;

    let result_str = result.to_string();
    let curr_num_str = curr_num.to_string();
    let mut concat_result: u64 = 0;
    let mut concat_flag = false;

    if result_str.ends_with(&curr_num_str) {
        let result_str = &result_str[..result_str.len() - curr_num_str.len()];
        concat_result = result_str.parse::<u64>().unwrap_or(0);
        concat_flag = true;
    }

    is_true_with_concat(input, sub_result, i - 1)
        || (should_div_path && is_true_with_concat(input, div_result, i - 1))
        || (concat_flag && is_true_with_concat(input, concat_result, i - 1))
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
