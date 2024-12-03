use std::{u32, usize};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = get_reports(input);
    let mut safe_reports_num: u32 = 0;
    for report in reports.iter() {
        if is_report_safe(report) {
            safe_reports_num += 1;
        }
    }

    Some(safe_reports_num)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = get_reports(input);
    let mut safe_reports_num: u32 = 0;

    for report in reports.iter() {
        if is_report_safe(report) {
            safe_reports_num += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut cloned_report = report.clone();
            cloned_report.remove(i);

            if is_report_safe(&cloned_report) {
                safe_reports_num += 1;
                break;
            }
        }
    }

    Some(safe_reports_num)
}

fn is_report_safe(report: &[u32]) -> bool {
    let is_asc: bool = report.first().unwrap() < report.get(1).unwrap();
    for i in 1..report.len() {
        if !is_diff_safe(is_asc, report, i - 1, i) {
            return false;
        }
    }
    true
}

fn get_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_diff_safe(is_asc: bool, report: &[u32], i: usize, j: usize) -> bool {
    if j >= report.len() {
        return true;
    }
    let first_element = report.get(i).unwrap();
    let second_element = report.get(j).unwrap();
    let abs_diff = first_element.abs_diff(*second_element);

    if !(1..=3).contains(&abs_diff) {
        return false;
    }

    if (is_asc && second_element < first_element) || (!is_asc && second_element > first_element) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
