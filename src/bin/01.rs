use std::{collections::HashMap, u32};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.split("\n").collect();

    let (mut first_column, mut second_column): (Vec<u32>, Vec<u32>) = rows
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            let mut parts = row.split_whitespace();
            {
                (
                    parts.next().unwrap().parse::<u32>().unwrap(),
                    parts.next().unwrap().parse::<u32>().unwrap(),
                )
            }
        })
        .unzip();

    first_column.sort();
    second_column.sort();

    let mut diff: u32 = 0;

    for (a, b) in first_column.iter().zip(second_column.iter()) {
        let curr_diff = a.abs_diff(*b);
        diff += curr_diff;
    }

    Some(diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.split("\n").collect();

    let mut first_column: Vec<u32> = Vec::new();
    let mut second_column: HashMap<u32, u32> = HashMap::new();

    for row in rows.iter().filter(|row| !row.is_empty()) {
        let mut parts = row.split_whitespace();
        let first = parts.next().unwrap().parse::<u32>().unwrap();
        let second = parts.next().unwrap().parse::<u32>().unwrap();

        first_column.push(first);
        *second_column.entry(second).or_insert(0) += 1;
    }

    let mut similarity_score: u32 = 0;

    for first_column_integer in first_column.iter() {
        similarity_score +=
            first_column_integer * second_column.get(first_column_integer).unwrap_or(&0);
    }

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
