advent_of_code::solution!(3);

use std::{u32, usize};

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let multiplies: Vec<_> = pattern.captures_iter(input).collect();

    let mut result: u32 = 0;
    for multiply in &multiplies {
        let first_num: u32 = multiply[1].parse().unwrap();
        let second_num: u32 = multiply[2].parse().unwrap();
        result += first_num * second_num;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let multiply_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let dos_pattern = Regex::new(r"do\(\)").unwrap();
    let donts_pattern = Regex::new(r"don\'t\(\)").unwrap();

    let multiplies: Vec<_> = multiply_pattern.captures_iter(input).collect();
    let dos: Vec<_> = dos_pattern.captures_iter(input).collect();
    let donts: Vec<_> = donts_pattern.captures_iter(input).collect();

    let mut multiplies_location: Vec<usize> = Vec::new();
    let mut multiplies_result: Vec<u32> = Vec::new();
    let mut dos_location: Vec<usize> = Vec::new();
    let mut donts_location: Vec<usize> = Vec::new();

    for multiply in &multiplies {
        let first_num: u32 = multiply[1].parse().unwrap();
        let second_num: u32 = multiply[2].parse().unwrap();
        multiplies_result.push(first_num * second_num);
        multiplies_location.push(multiply.get(0).unwrap().start());
    }

    for doo in &dos {
        dos_location.push(doo.get(0).unwrap().start());
    }

    for dont in &donts {
        donts_location.push(dont.get(0).unwrap().start());
    }

    let mut result: u32 = 0;

    let mut do_i: usize = usize::MAX;
    let mut dont_i: usize = usize::MAX;

    for i in 0..multiplies_location.len() {
        let curr_location = multiplies_location.get(i).unwrap();

        for j in 0..dos_location.len() {
            if dos_location.get(j).unwrap() < curr_location {
                do_i = *dos_location.get(j).unwrap();
            } else {
                break;
            }
        }

        for j in 0..donts_location.len() {
            if donts_location.get(j).unwrap() < curr_location {
                dont_i = *donts_location.get(j).unwrap();
            } else {
                break;
            }
        }

        if curr_location.abs_diff(do_i) <= curr_location.abs_diff(dont_i) {
            result += multiplies_result.get(i).unwrap();
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
